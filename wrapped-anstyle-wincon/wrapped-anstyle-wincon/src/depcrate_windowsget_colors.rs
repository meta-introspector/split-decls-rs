// Generated macro for get_colors (function)
macro_rules! Depcrate_windowsget_colors {
() => {
// Module: crate::windows
// Provides: {"get_colors"}
// Dependencies: {}
# [doc = " Get the colors currently active on the console"] pub fn get_colors < S : AsHandle > (stream : & S) -> StdioColorResult { get_colors_ (stream) . map_err (Into :: into) }
};
}
