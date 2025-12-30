// Generated macro for decode (function)
macro_rules! Depcrate_upperdecode {
() => {
// Module: crate::upper
// Provides: {"decode"}
// Dependencies: {}
# [doc = " Decode an upper Base16 (hex) string into the provided destination buffer."] pub fn decode (src : impl AsRef < [u8] > , dst : & mut [u8]) -> Result < & [u8] , Error > { decode_inner (src . as_ref () , dst , decode_nibble) }
};
}
