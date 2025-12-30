// Generated macro for decoded_length (function)
macro_rules! Depcrate_base64decoded_length {
() => {
// Module: crate::base64
// Provides: {"decoded_length"}
// Dependencies: {}
# [doc = " Provide an upper limit on how much space could be required"] # [doc = " to decode a base64 encoding of len `base64_len`."] pub (crate) const fn decoded_length (base64_len : usize) -> usize { ((base64_len + 3) / 4) * 3 }
};
}
