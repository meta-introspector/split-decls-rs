// Generated macro for hf64 (function)
macro_rules! Depcrate_math_support_hex_floathf64 {
() => {
// Module: crate::math::support::hex_float
// Provides: {"hf64"}
// Dependencies: {}
# [doc = " Construct a 64-bit float from hex float representation (C-style)"] pub const fn hf64 (s : & str) -> f64 { match parse_hex_exact (s , 64 , 52) { Ok (bits) => f64_from_bits (bits as u64) , Err (HexFloatParseError (s)) => panic ! ("{}" , s) , } }
};
}
