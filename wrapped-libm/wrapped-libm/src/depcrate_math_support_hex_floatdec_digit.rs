// Generated macro for dec_digit (function)
macro_rules! Depcrate_math_support_hex_floatdec_digit {
() => {
// Module: crate::math::support::hex_float
// Provides: {"dec_digit"}
// Dependencies: {}
const fn dec_digit (c : u8) -> Option < u8 > { match c { b'0' ..= b'9' => Some (c - b'0') , _ => None , } }
};
}
