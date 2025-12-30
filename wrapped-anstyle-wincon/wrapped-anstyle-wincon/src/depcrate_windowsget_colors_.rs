// Generated macro for get_colors_ (function)
macro_rules! Depcrate_windowsget_colors_ {
() => {
// Module: crate::windows
// Provides: {"get_colors_"}
// Dependencies: {}
fn get_colors_ < S : AsHandle > (stream : & S) -> StdioColorInnerResult { let handle = stream . as_handle () ; let handle = handle . as_raw_handle () ; let info = inner :: get_screen_buffer_info (handle) ? ; let (fg , bg) = inner :: get_colors (& info) ; Ok ((fg , bg)) }
};
}
