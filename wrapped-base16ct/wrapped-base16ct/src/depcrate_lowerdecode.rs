// Generated macro for decode (function)
macro_rules! Depcrate_lowerdecode {
() => {
// Module: crate::lower
// Provides: {"decode"}
// Dependencies: {}
# [doc = " Decode a lower Base16 (hex) string into the provided destination buffer."] pub fn decode (src : impl AsRef < [u8] > , dst : & mut [u8]) -> Result < & [u8] , Error > { decode_inner (src . as_ref () , dst , decode_nibble) }
};
}
