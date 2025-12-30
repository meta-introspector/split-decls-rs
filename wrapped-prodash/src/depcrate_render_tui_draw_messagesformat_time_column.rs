// Generated macro for format_time_column (function)
macro_rules! Depcrate_render_tui_draw_messagesformat_time_column {
() => {
// Module: crate::render::tui::draw::messages
// Provides: {"format_time_column"}
// Dependencies: {}
fn format_time_column (time : & SystemTime) -> String { format ! ("{}{}" , format_time_for_messages (* time) , VERTICAL_LINE) }
};
}
