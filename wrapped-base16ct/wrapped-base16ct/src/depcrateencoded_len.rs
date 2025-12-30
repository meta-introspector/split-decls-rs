// Generated macro for encoded_len (function)
macro_rules! Depcrateencoded_len {
() => {
// Module: crate
// Provides: {"encoded_len"}
// Dependencies: {}
# [doc = " Get the length of Base16 (hex) produced by encoding the given bytes."] # [inline (always)] pub fn encoded_len (bytes : & [u8]) -> usize { bytes . len () * 2 }
};
}
