// Generated macro for hex_digit (function)
macro_rules! Depcrate_math_support_hex_floathex_digit {
() => {
// Module: crate::math::support::hex_float
// Provides: {"hex_digit"}
// Dependencies: {}
const fn hex_digit (c : u8) -> Option < u8 > { match c { b'0' ..= b'9' => Some (c - b'0') , b'a' ..= b'f' => Some (c - b'a' + 10) , b'A' ..= b'F' => Some (c - b'A' + 10) , _ => None , } }
};
}
