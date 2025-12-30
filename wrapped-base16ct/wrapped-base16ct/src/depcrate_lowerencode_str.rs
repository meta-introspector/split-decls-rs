// Generated macro for encode_str (function)
macro_rules! Depcrate_lowerencode_str {
() => {
// Module: crate::lower
// Provides: {"encode_str"}
// Dependencies: {}
# [doc = " Encode input byte slice into a [`&str`] containing lower Base16 (hex)."] pub fn encode_str < 'a > (src : & [u8] , dst : & 'a mut [u8]) -> Result < & 'a str , Error > { encode (src , dst) . map (| r | unsafe { core :: str :: from_utf8_unchecked (r) }) }
};
}
