// Generated macro for encode_str (function)
macro_rules! Depcrate_upperencode_str {
() => {
// Module: crate::upper
// Provides: {"encode_str"}
// Dependencies: {}
# [doc = " Encode input byte slice into a [`&str`] containing upper Base16 (hex)."] pub fn encode_str < 'a > (src : & [u8] , dst : & 'a mut [u8]) -> Result < & 'a str , Error > { encode (src , dst) . map (| r | unsafe { core :: str :: from_utf8_unchecked (r) }) }
};
}
