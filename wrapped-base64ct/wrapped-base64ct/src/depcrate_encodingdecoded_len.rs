// Generated macro for decoded_len (function)
macro_rules! Depcrate_encodingdecoded_len {
() => {
// Module: crate::encoding
// Provides: {"decoded_len"}
// Dependencies: {}
# [doc = " Get the length of the output from decoding the provided *unpadded*"] # [doc = " Base64-encoded input."] # [doc = ""] # [doc = " Note that this function does not fully validate the Base64 is well-formed"] # [doc = " and may return incorrect results for malformed Base64."] # [allow (clippy :: arithmetic_side_effects)] # [inline (always)] pub (crate) fn decoded_len (input_len : usize) -> usize { let k = input_len / 4 ; let l = input_len - 4 * k ; 3 * k + (3 * l) / 4 }
};
}
