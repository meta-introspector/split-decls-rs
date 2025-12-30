// Generated macro for decode_to_vec (function)
macro_rules! Depcrate_forgiving_base64decode_to_vec {
() => {
// Module: crate::forgiving_base64
// Provides: {"decode_to_vec"}
// Dependencies: {}
# [doc = " `input` is assumed to be in an ASCII-compatible encoding"] pub fn decode_to_vec (input : & [u8]) -> Result < Vec < u8 > , InvalidBase64 > { let mut v = Vec :: new () ; { let mut decoder = Decoder :: new (| bytes | { v . extend_from_slice (bytes) ; Ok (()) }) ; decoder . feed (input) ? ; decoder . finish () ? ; } Ok (v) }
};
}
