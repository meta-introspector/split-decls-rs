// Generated macro for encode_string (function)
macro_rules! Depcrate_encoderencode_string {
() => {
// Module: crate::encoder
// Provides: {"encode_string"}
// Dependencies: {}
# [doc = " Encode a PEM document according to RFC 7468's \"Strict\" grammar, returning"] # [doc = " the result as a [`String`]."] # [cfg (feature = "alloc")] pub fn encode_string (label : & str , line_ending : LineEnding , input : & [u8]) -> Result < String > { let expected_len = encoded_len (label , line_ending , input) ? ; let mut buf = vec ! [0u8 ; expected_len] ; let actual_len = encode (label , line_ending , input , & mut buf) ? . len () ; debug_assert_eq ! (expected_len , actual_len) ; String :: from_utf8 (buf) . map_err (| _ | Error :: CharacterEncoding) }
};
}
