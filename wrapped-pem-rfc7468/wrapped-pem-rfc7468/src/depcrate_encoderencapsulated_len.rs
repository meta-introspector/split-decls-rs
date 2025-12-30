// Generated macro for encapsulated_len (function)
macro_rules! Depcrate_encoderencapsulated_len {
() => {
// Module: crate::encoder
// Provides: {"encapsulated_len"}
// Dependencies: {}
# [doc = " Compute the length of a PEM encoded document which encapsulates a"] # [doc = " Base64-encoded body including line endings every 64 characters."] # [doc = ""] # [doc = " The `input_len` parameter specifies the length of the raw input"] # [doc = " bytes prior to Base64 encoding."] # [doc = ""] # [doc = " Note that the current implementation of this function computes an upper"] # [doc = " bound of the length and the actual encoded document may be slightly shorter"] # [doc = " (typically 1-byte). Downstream consumers of this function should check the"] # [doc = " actual encoded length and potentially truncate buffers allocated using this"] # [doc = " function to estimate the encapsulated size."] # [doc = ""] # [doc = " Use [`encoded_len`] (when possible) to obtain a precise length."] # [doc = ""] # [doc = " ## Returns"] # [doc = " - `Ok(len)` on success"] # [doc = " - `Err(Error::Length)` on length overflow"] pub fn encapsulated_len (label : & str , line_ending : LineEnding , input_len : usize) -> Result < usize > { encapsulated_len_wrapped (label , BASE64_WRAP_WIDTH , line_ending , input_len) }
};
}
