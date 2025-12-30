// Generated macro for utf16_from_utf8 (function)
macro_rules! Depcrate_testingutf16_from_utf8 {
() => {
// Module: crate::testing
// Provides: {"utf16_from_utf8"}
// Dependencies: {}
pub fn utf16_from_utf8 (string : & str) -> Vec < u16 > { let mut decoder = UTF_8 . new_decoder_without_bom_handling () ; let mut vec = Vec :: with_capacity (decoder . max_utf16_buffer_length (string . len ()) . unwrap ()) ; let capacity = vec . capacity () ; vec . resize (capacity , 0) ; let (result , read , written) = decoder . decode_to_utf16_without_replacement (string . as_bytes () , & mut vec [..] , true) ; match result { DecoderResult :: InputEmpty => { debug_assert_eq ! (read , string . len ()) ; vec . resize (written , 0) ; vec } DecoderResult :: Malformed (_ , _) => unreachable ! ("Malformed") , DecoderResult :: OutputFull => unreachable ! ("Output full") , } }
};
}
