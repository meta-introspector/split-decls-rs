// Generated macro for is_a_key_char (function)
macro_rules! Depcrate_parsers_grammaris_a_key_char {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_a_key_char"}
// Dependencies: {}
# [doc = " Checks if ascii char is an `AKeyChar`."] # [inline] pub (crate) const fn is_a_key_char (ch : u8) -> bool { is_a_key_leading_char (ch) || ch . is_ascii_digit () || ch == b'-' }
};
}
