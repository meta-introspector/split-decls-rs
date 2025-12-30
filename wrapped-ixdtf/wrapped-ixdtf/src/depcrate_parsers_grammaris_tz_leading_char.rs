// Generated macro for is_tz_leading_char (function)
macro_rules! Depcrate_parsers_grammaris_tz_leading_char {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_tz_leading_char"}
// Dependencies: {}
# [doc = " Checks if ascii char is a `TZLeadingChar`."] # [inline] pub (crate) const fn is_tz_leading_char (ch : u8) -> bool { ch . is_ascii_alphabetic () || ch == b'_' || ch == b'.' }
};
}
