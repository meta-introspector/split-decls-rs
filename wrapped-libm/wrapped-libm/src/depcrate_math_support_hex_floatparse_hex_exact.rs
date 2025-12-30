// Generated macro for parse_hex_exact (function)
macro_rules! Depcrate_math_support_hex_floatparse_hex_exact {
() => {
// Module: crate::math::support::hex_float
// Provides: {"parse_hex_exact"}
// Dependencies: {}
# [doc = " Parses any float to its bitwise representation, returning an error if it cannot be represented exactly"] pub const fn parse_hex_exact (s : & str , bits : u32 , sig_bits : u32 ,) -> Result < u128 , HexFloatParseError > { match parse_any (s , bits , sig_bits , Round :: Nearest) { Err (e) => Err (e) , Ok ((bits , Status :: OK)) => Ok (bits) , Ok ((_ , status)) if status . overflow () => Err (HexFloatParseError ("the value is too huge")) , Ok ((_ , status)) if status . underflow () => Err (HexFloatParseError ("the value is too tiny")) , Ok ((_ , status)) if status . inexact () => Err (HexFloatParseError ("the value is too precise")) , Ok (_) => unreachable ! () , } }
};
}
