// Generated macro for impl_215 (impl)
macro_rules! Depcrate_utf_8impl_215 {
() => {
// Module: crate::utf_8
// Provides: {"impl_215"}
// Dependencies: {}
impl Utf8Encoder { pub fn new (encoding : & 'static Encoding) -> Encoder { Encoder :: new (encoding , VariantEncoder :: Utf8 (Utf8Encoder)) } pub fn max_buffer_length_from_utf16_without_replacement (& self , u16_length : usize ,) -> Option < usize > { u16_length . checked_mul (3) } pub fn max_buffer_length_from_utf8_without_replacement (& self , byte_length : usize ,) -> Option < usize > { Some (byte_length) } pub fn encode_from_utf16_raw (& mut self , src : & [u16] , dst : & mut [u8] , _last : bool ,) -> (EncoderResult , usize , usize) { let (read , written) = convert_utf16_to_utf8_partial (src , dst) ; (if read == src . len () { EncoderResult :: InputEmpty } else { EncoderResult :: OutputFull } , read , written ,) } pub fn encode_from_utf8_raw (& mut self , src : & str , dst : & mut [u8] , _last : bool ,) -> (EncoderResult , usize , usize) { let bytes = src . as_bytes () ; let mut to_write = bytes . len () ; if to_write <= dst . len () { (& mut dst [.. to_write]) . copy_from_slice (bytes) ; return (EncoderResult :: InputEmpty , to_write , to_write) ; } to_write = dst . len () ; while (bytes [to_write] & 0xC0) == 0x80 { to_write -= 1 ; } (& mut dst [.. to_write]) . copy_from_slice (& bytes [.. to_write]) ; (EncoderResult :: OutputFull , to_write , to_write) } }
};
}
