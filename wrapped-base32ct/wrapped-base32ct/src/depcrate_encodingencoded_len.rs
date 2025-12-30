// Generated macro for encoded_len (function)
macro_rules! Depcrate_encodingencoded_len {
() => {
// Module: crate::encoding
// Provides: {"encoded_len"}
// Dependencies: {}
# [doc = " Get the length of Base32 produced by encoding the given amount of bytes."] pub const fn encoded_len < T : Encoding > (length : usize) -> usize { if length == 0 { 0 } else if T :: PADDED { ((length - 1) / 5 + 1) * 8 } else { (length * 8) . div_ceil (5) } }
};
}
