// Generated macro for legal_char_start (function)
macro_rules! Depcrate_unicodeset_parse_parselegal_char_start {
() => {
// Module: crate::unicodeset_parse::parse
// Provides: {"legal_char_start"}
// Dependencies: {}
fn legal_char_start (c : char) -> bool { ! (c == '&' || c == '-' || c == '$' || c == '^' || c == '[' || c == ']' || c == '{') }
};
}
