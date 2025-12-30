// Generated macro for strip_ws_lines (function)
macro_rules! Depcrate_frontmatterstrip_ws_lines {
() => {
// Module: crate::frontmatter
// Provides: {"strip_ws_lines"}
// Dependencies: {}
# [doc = " Returns the index after any lines with only whitespace, if present"] pub fn strip_ws_lines (input : & str) -> Option < usize > { let ws_end = input . find (| c | ! is_whitespace (c)) . unwrap_or (input . len ()) ; if ws_end == 0 { return None ; } let nl_start = input [0 .. ws_end] . rfind ('\n') ? ; let nl_end = nl_start + 1 ; Some (nl_end) }
};
}
