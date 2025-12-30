// Generated macro for indent_of_nth_line (function)
macro_rules! Depcrate_matches_match_single_bindingindent_of_nth_line {
() => {
// Module: crate::matches::match_single_binding
// Provides: {"indent_of_nth_line"}
// Dependencies: {}
fn indent_of_nth_line (snippet : & str , nth : usize) -> Option < usize > { snippet . lines () . nth (nth) . and_then (| s | s . find (| c : char | ! c . is_whitespace ())) }
};
}
