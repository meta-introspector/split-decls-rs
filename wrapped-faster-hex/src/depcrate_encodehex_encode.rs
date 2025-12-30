// Generated macro for hex_encode (function)
macro_rules! Depcrate_encodehex_encode {
() => {
// Module: crate::encode
// Provides: {"hex_encode"}
// Dependencies: {}
# [doc = " Hex encode src into dst."] # [doc = " The length of dst must be at least src.len() * 2."] pub fn hex_encode < 'a > (src : & [u8] , dst : & 'a mut [u8]) -> Result < & 'a mut str , Error > { hex_encode_custom (src , dst , false) }
};
}
