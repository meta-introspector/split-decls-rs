// Generated macro for base64_encoded_len (function)
macro_rules! Depcrate_basicbase64_encoded_len {
() => {
// Module: crate::basic
// Provides: {"base64_encoded_len"}
// Dependencies: {}
# [doc = " Returns the base64-encoded length for the given input length, including padding."] fn base64_encoded_len (input_len : usize) -> usize { (input_len + 2) / 3 * 4 }
};
}
