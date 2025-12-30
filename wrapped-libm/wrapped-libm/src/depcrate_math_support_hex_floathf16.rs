// Generated macro for hf16 (function)
macro_rules! Depcrate_math_support_hex_floathf16 {
() => {
// Module: crate::math::support::hex_float
// Provides: {"hf16"}
// Dependencies: {}
# [doc = " Construct a 16-bit float from hex float representation (C-style)"] # [cfg (f16_enabled)] pub const fn hf16 (s : & str) -> f16 { match parse_hex_exact (s , 16 , 10) { Ok (bits) => f16 :: from_bits (bits as u16) , Err (HexFloatParseError (s)) => panic ! ("{}" , s) , } }
};
}
