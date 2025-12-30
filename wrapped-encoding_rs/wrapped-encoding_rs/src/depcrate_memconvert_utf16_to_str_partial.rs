// Generated macro for convert_utf16_to_str_partial (function)
macro_rules! Depcrate_memconvert_utf16_to_str_partial {
() => {
// Module: crate::mem
// Provides: {"convert_utf16_to_str_partial"}
// Dependencies: {}
# [doc = " Converts potentially-invalid UTF-16 to valid UTF-8 with errors replaced"] # [doc = " with the REPLACEMENT CHARACTER such that the validity of the output is"] # [doc = " signaled using the Rust type system with potentially insufficient output"] # [doc = " space."] # [doc = ""] # [doc = " Returns the number of code units read and the number of bytes written."] # [doc = ""] # [doc = " Not all code units are read if there isn't enough output space."] # [doc = ""] # [doc = " Note  that this method isn't designed for general streamability but for"] # [doc = " not allocating memory for the worst case up front. Specifically,"] # [doc = " if the input starts with or ends with an unpaired surrogate, those are"] # [doc = " replaced with the REPLACEMENT CHARACTER."] pub fn convert_utf16_to_str_partial (src : & [u16] , dst : & mut str) -> (usize , usize) { let bytes : & mut [u8] = unsafe { dst . as_bytes_mut () } ; let (read , written) = convert_utf16_to_utf8_partial (src , bytes) ; let len = bytes . len () ; let mut trail = written ; while trail < len && ((bytes [trail] & 0xC0) == 0x80) { bytes [trail] = 0 ; trail += 1 ; } (read , written) }
};
}
