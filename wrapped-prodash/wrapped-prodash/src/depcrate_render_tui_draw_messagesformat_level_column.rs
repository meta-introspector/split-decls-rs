// Generated macro for format_level_column (function)
macro_rules! Depcrate_render_tui_draw_messagesformat_level_column {
() => {
// Module: crate::render::tui::draw::messages
// Provides: {"format_level_column"}
// Dependencies: {}
fn format_level_column (level : MessageLevel) -> & 'static str { use MessageLevel :: * ; match level { Info => "info" , Failure => "fail" , Success => "done" , } }
};
}
