// Generated macro for is_tz_char (function)
macro_rules! Depcrate_parsers_grammaris_tz_char {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_tz_char"}
// Dependencies: {}
# [doc = " Checks if ascii char is a `TZChar`."] # [inline] pub (crate) const fn is_tz_char (ch : u8) -> bool { is_tz_leading_char (ch) || ch . is_ascii_digit () || ch == b'-' || ch == b'+' }
};
}
