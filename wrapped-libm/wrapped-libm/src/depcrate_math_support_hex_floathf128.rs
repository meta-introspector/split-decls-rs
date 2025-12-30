// Generated macro for hf128 (function)
macro_rules! Depcrate_math_support_hex_floathf128 {
() => {
// Module: crate::math::support::hex_float
// Provides: {"hf128"}
// Dependencies: {}
# [doc = " Construct a 128-bit float from hex float representation (C-style)"] # [cfg (f128_enabled)] pub const fn hf128 (s : & str) -> f128 { match parse_hex_exact (s , 128 , 112) { Ok (bits) => f128 :: from_bits (bits) , Err (HexFloatParseError (s)) => panic ! ("{}" , s) , } }
};
}
