// Generated macro for hex_decode (function)
macro_rules! Depcrate_decodehex_decode {
() => {
// Module: crate::decode
// Provides: {"hex_decode"}
// Dependencies: {}
# [doc = " Hex decode src into dst."] # [doc = " The length of src must be even, and it's allowed to decode a zero length src."] # [doc = " The length of dst must be at least src.len() / 2."] pub fn hex_decode (src : & [u8] , dst : & mut [u8]) -> Result < () , Error > { hex_decode_with_case (src , dst , CheckCase :: None) }
};
}
