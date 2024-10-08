// All that is fixed in place in the foreground
// Excludes the graph and floating windows(?)

use bevy::{ecs::identifier, prelude::*, render::view::VisibleEntities, ui::{FocusPolicy, UiSystem}};

use bevy_mod_picking::backends::raycast::RaycastBackendSettings;
use bevy_prototype_lyon::prelude::*;
use edges::EdgeUiPlugin;

use self::{
    nodes::NodesUiPlugin, // edges::EdgeUiPlugin, 
    grid::InfiniteGrid2DPlugin, graph_cam::GraphCamera, asset_manager::{ImageLoadTracker, on_image_load},
};

use super::events::{node_events::NodeClickEvent, edges::EdgeClickEvent};

// Building blocks of specific components

pub(crate) mod grid;
pub(crate) mod ui_base_panel;
pub(crate) mod nodes;
pub(crate) mod edges;
pub(crate) mod graph_cam;
pub(crate) mod simulation;
pub(crate) mod asset_manager;
pub(crate) mod curve_material;

pub struct KartaUiPlugin;

impl Plugin for KartaUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins(ShapePlugin)

            .add_plugins(graph_cam::GraphCamPlugin)
            .add_plugins(simulation::GraphSimPlugin)

            .add_plugins(ui_base_panel::UiNodePlugin)

            .insert_resource(ImageLoadTracker::new())

            // .add_event::<ContextMenuSpawnEvent>()

            // Resources
            .add_systems(PreStartup, require_markers_for_raycasting)
            // .add_systems(PreStartup, default_font_setup)
            // .add_systems(PreStartup, 
            //     default_font_set.run_if(resource_exists::<FontHandle>()))

            .add_systems(Startup, gizmo_settings)

            .add_plugins(NodesUiPlugin)
            .add_plugins(EdgeUiPlugin)
            // .add_plugins(InfiniteGrid2DPlugin)
            
            // Element Systems
            // .add_systems(PostUpdate, popup::popup_position_system.after(UiSystem::Layout))
                        
            
            // .add_systems(Update, context_menu::despawn_context_menus_on_any_click)
            // .add_systems(
            //     Update, 
            //     (
            //         spawn_node_context_menu.run_if(on_event::<NodeClickEvent>()),
            //         spawn_edge_context_menu.run_if(on_event::<EdgeClickEvent>())
            //     )
            //     // .after(context_menu::despawn_context_menus_on_any_click)
            //     // .before(add_component_systems_to_context_menu)
            // )

            // .add_systems(PostUpdate, add_component_systems_to_context_menu
            //     .run_if(on_event::<ContextMenuSpawnEvent>())
            // ) 
            
            // .add_systems(PostUpdate, context_menu_button_system)
            .add_systems(PostUpdate, on_image_load)

            .add_plugins(curve_material::CurveMaterialPlugin)
        ;
    }
}

fn default_font_set(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
    asset_server: Res<AssetServer>,
    font_handle: Res<FontHandle>,
){
    if let Some(font) = fonts.remove(&font_handle.0) {
        fonts.add(font);
        commands.remove_resource::<FontHandle>();
    }

    let font = asset_server.load("fonts/Roboto/Roboto-Medium.ttf");
    commands.insert_resource(FontHandle(font));
}

fn default_font_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

) {
    let font = asset_server.load("fonts/Roboto/Roboto-Medium.ttf");
    commands.insert_resource(FontHandle(font));
}

fn require_markers_for_raycasting(
    mut settings: ResMut<RaycastBackendSettings>,
){
    settings.require_markers = true;
}

#[derive(Resource)]
struct FontHandle(Handle<Font>);

fn gizmo_settings(
    mut gizmo: ResMut<GizmoConfigStore>,
){
    // let mut gizmo = gizmo.get_config_mut_dyn(<DefaultGizmoConfigGroup>::identifier);
    // gizmo.depth_bias = 1.0;::
    // gizmo.render_layers = bevy::render::view::RenderLayers::layer(31);
}