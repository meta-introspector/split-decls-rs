// Generated macro for is_multiline_string_debug (function)
macro_rules! Depcrate_matchers_eq_matcheris_multiline_string_debug {
() => {
// Module: crate::matchers::eq_matcher
// Provides: {"is_multiline_string_debug"}
// Dependencies: {}
fn is_multiline_string_debug (string : & str) -> bool { string . starts_with ('"') && string . ends_with ('"') && ! string . contains ('\n') && string . contains ("\\n") }
};
}
