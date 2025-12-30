// Generated macro for encoded_len (function)
macro_rules! Depcrate_encoderencoded_len {
() => {
// Module: crate::encoder
// Provides: {"encoded_len"}
// Dependencies: {}
# [doc = " Get the length of a PEM encoded document with the given bytes and label."] # [doc = ""] # [doc = " This function computes a precise length of the PEM encoding of the given"] # [doc = " `input` data."] # [doc = ""] # [doc = " ## Returns"] # [doc = " - `Ok(len)` on success"] # [doc = " - `Err(Error::Length)` on length overflow"] pub fn encoded_len (label : & str , line_ending : LineEnding , input : & [u8]) -> Result < usize > { let base64_len = Base64 :: encoded_len (input) ; let base64_len_wrapped = base64_len_wrapped (base64_len , BASE64_WRAP_WIDTH , line_ending) ? ; encapsulated_len_inner (label , line_ending , base64_len_wrapped) }
};
}
