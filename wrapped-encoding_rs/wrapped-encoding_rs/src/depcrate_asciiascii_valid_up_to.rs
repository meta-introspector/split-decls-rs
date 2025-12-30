// Generated macro for ascii_valid_up_to (function)
macro_rules! Depcrate_asciiascii_valid_up_to {
() => {
// Module: crate::ascii
// Provides: {"ascii_valid_up_to"}
// Dependencies: {}
pub fn ascii_valid_up_to (bytes : & [u8]) -> usize { match validate_ascii (bytes) { None => bytes . len () , Some ((_ , num_valid)) => num_valid , } }
};
}
