// Generated macro for is_a_key_leading_char (function)
macro_rules! Depcrate_parsers_grammaris_a_key_leading_char {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_a_key_leading_char"}
// Dependencies: {}
# [doc = " Checks if ascii char is a `AKeyLeadingChar`."] # [inline] pub (crate) const fn is_a_key_leading_char (ch : u8) -> bool { ch . is_ascii_lowercase () || ch == b'_' }
};
}
