// Generated macro for convert_utf8_to_utf16 (function)
macro_rules! Depcrate_memconvert_utf8_to_utf16 {
() => {
// Module: crate::mem
// Provides: {"convert_utf8_to_utf16"}
// Dependencies: {}
# [doc = " Converts potentially-invalid UTF-8 to valid UTF-16 with errors replaced"] # [doc = " with the REPLACEMENT CHARACTER."] # [doc = ""] # [doc = " The length of the destination buffer must be at least the length of the"] # [doc = " source buffer _plus one_."] # [doc = ""] # [doc = " Returns the number of `u16`s written."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the destination buffer is shorter than stated above."] pub fn convert_utf8_to_utf16 (src : & [u8] , dst : & mut [u16]) -> usize { assert ! (dst . len () > src . len ()) ; let mut decoder = Utf8Decoder :: new_inner () ; let mut total_read = 0usize ; let mut total_written = 0usize ; loop { let (result , read , written) = decoder . decode_to_utf16_raw (& src [total_read ..] , & mut dst [total_written ..] , true) ; total_read += read ; total_written += written ; match result { DecoderResult :: InputEmpty => { return total_written ; } DecoderResult :: OutputFull => { unreachable ! ("The assert at the top of the function should have caught this.") ; } DecoderResult :: Malformed (_ , _) => { dst [total_written] = 0xFFFD ; total_written += 1 ; } } } }
};
}
