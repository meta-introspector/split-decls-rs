// Generated macro for is_ascii_sign (function)
macro_rules! Depcrate_parsers_grammaris_ascii_sign {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_ascii_sign"}
// Dependencies: {}
# [doc = " Checks if ascii char is an ascii sign."] # [inline] pub (crate) const fn is_ascii_sign (ch : u8) -> bool { ch == b'+' || ch == b'-' }
};
}
