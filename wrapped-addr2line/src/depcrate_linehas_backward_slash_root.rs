// Generated macro for has_backward_slash_root (function)
macro_rules! Depcrate_linehas_backward_slash_root {
() => {
// Module: crate::line
// Provides: {"has_backward_slash_root"}
// Dependencies: {}
# [doc = " Check if the path in the given string has a windows style root"] fn has_backward_slash_root (p : & str) -> bool { p . starts_with ('\\') || p . get (1 .. 3) == Some (":\\") }
};
}
