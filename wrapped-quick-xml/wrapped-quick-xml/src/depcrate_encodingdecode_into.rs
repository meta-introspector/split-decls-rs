// Generated macro for decode_into (function)
macro_rules! Depcrate_encodingdecode_into {
() => {
// Module: crate::encoding
// Provides: {"decode_into"}
// Dependencies: {}
# [doc = " Like [`decode`] but using a pre-allocated buffer."] # [cfg (feature = "encoding")] pub fn decode_into (bytes : & [u8] , encoding : & 'static Encoding , buf : & mut String ,) -> Result < () , EncodingError > { if encoding == UTF_8 { buf . push_str (std :: str :: from_utf8 (bytes) ?) ; return Ok (()) ; } let mut decoder = encoding . new_decoder_without_bom_handling () ; buf . reserve (decoder . max_utf8_buffer_length_without_replacement (bytes . len ()) . unwrap () ,) ; let (result , read) = decoder . decode_to_string_without_replacement (bytes , buf , true) ; match result { DecoderResult :: InputEmpty => { debug_assert_eq ! (read , bytes . len ()) ; Ok (()) } DecoderResult :: Malformed (_ , _) => Err (EncodingError :: Other (encoding)) , DecoderResult :: OutputFull => unreachable ! () , } }
};
}
