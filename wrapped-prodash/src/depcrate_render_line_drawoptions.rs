// Generated macro for Options (struct)
macro_rules! Depcrate_render_line_drawOptions {
() => {
// Module: crate::render::line::draw
// Provides: {"Options"}
// Dependencies: {}
pub struct Options { pub level_filter : Option < RangeInclusive < progress :: key :: Level > > , pub terminal_dimensions : (u16 , u16) , pub keep_running_if_progress_is_empty : bool , pub output_is_terminal : bool , pub colored : bool , pub timestamp : bool , pub hide_cursor : bool , }
};
}
