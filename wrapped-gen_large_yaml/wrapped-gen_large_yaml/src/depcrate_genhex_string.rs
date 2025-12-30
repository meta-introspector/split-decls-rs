// Generated macro for hex_string (function)
macro_rules! Depcrate_genhex_string {
() => {
// Module: crate::gen
// Provides: {"hex_string"}
// Dependencies: {}
# [doc = " Generate a string with hexadecimal digits of the specified length."] pub fn hex_string (rng : & mut SmallRng , len : usize) -> String { const DIGITS : & [u8] = b"0123456789abcdef" ; string_from_set (rng , len , len + 1 , DIGITS) }
};
}
