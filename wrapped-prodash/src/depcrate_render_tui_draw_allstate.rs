// Generated macro for State (struct)
macro_rules! Depcrate_render_tui_draw_allState {
() => {
// Module: crate::render::tui::draw::all
// Provides: {"State"}
// Dependencies: {}
# [derive (Default)] pub struct State { pub title : String , pub task_offset : u16 , pub message_offset : u16 , pub hide_messages : bool , pub messages_fullscreen : bool , pub user_provided_window_size : Option < Rect > , pub duration_per_frame : Duration , pub information : Vec < Line > , pub hide_info : bool , pub maximize_info : bool , pub last_tree_column_width : Option < u16 > , pub next_tree_column_width : Option < u16 > , pub throughput : Option < Throughput > , }
};
}
