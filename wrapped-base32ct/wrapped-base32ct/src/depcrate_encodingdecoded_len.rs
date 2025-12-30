// Generated macro for decoded_len (function)
macro_rules! Depcrate_encodingdecoded_len {
() => {
// Module: crate::encoding
// Provides: {"decoded_len"}
// Dependencies: {}
# [doc = " Get the length of the output from decoding the provided *unpadded*"] # [doc = " Base32-encoded input."] # [doc = ""] # [doc = " Note that this function does not fully validate the Base32 is well-formed"] # [doc = " and may return incorrect results for malformed Base32."] # [inline (always)] fn decoded_len (input_len : usize) -> usize { (input_len * 5) / 8 }
};
}
