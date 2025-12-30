// Generated macro for impl_163 (impl)
macro_rules! Depcrate_replacementimpl_163 {
() => {
// Module: crate::replacement
// Provides: {"impl_163"}
// Dependencies: {}
impl ReplacementDecoder { pub fn new () -> VariantDecoder { VariantDecoder :: Replacement (ReplacementDecoder { emitted : false }) } pub fn max_utf16_buffer_length (& self , _u16_length : usize) -> Option < usize > { Some (1) } pub fn max_utf8_buffer_length_without_replacement (& self , _byte_length : usize) -> Option < usize > { Some (3) } pub fn max_utf8_buffer_length (& self , _byte_length : usize) -> Option < usize > { Some (3) } pub fn decode_to_utf16_raw (& mut self , src : & [u8] , dst : & mut [u16] , _last : bool ,) -> (DecoderResult , usize , usize) { if self . emitted || src . is_empty () { (DecoderResult :: InputEmpty , src . len () , 0) } else if dst . is_empty () { (DecoderResult :: OutputFull , 0 , 0) } else { self . emitted = true ; (DecoderResult :: Malformed (1 , 0) , 1 , 0) } } pub fn decode_to_utf8_raw (& mut self , src : & [u8] , dst : & mut [u8] , _last : bool ,) -> (DecoderResult , usize , usize) { if self . emitted || src . is_empty () { (DecoderResult :: InputEmpty , src . len () , 0) } else if dst . len () < 3 { (DecoderResult :: OutputFull , 0 , 0) } else { self . emitted = true ; (DecoderResult :: Malformed (1 , 0) , 1 , 0) } } }
};
}
