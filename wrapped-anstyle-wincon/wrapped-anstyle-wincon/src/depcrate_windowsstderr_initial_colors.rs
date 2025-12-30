// Generated macro for stderr_initial_colors (function)
macro_rules! Depcrate_windowsstderr_initial_colors {
() => {
// Module: crate::windows
// Provides: {"stderr_initial_colors"}
// Dependencies: {}
# [doc = " Cached [`get_colors`] call for [`std::io::stderr`]"] pub fn stderr_initial_colors () -> StdioColorResult { static INITIAL : once_cell_polyfill :: sync :: OnceLock < StdioColorInnerResult > = once_cell_polyfill :: sync :: OnceLock :: new () ; (* INITIAL . get_or_init (| | get_colors_ (& std :: io :: stderr ()))) . map_err (Into :: into) }
};
}
