// Generated macro for decode_vec (function)
macro_rules! Depcrate_decoderdecode_vec {
() => {
// Module: crate::decoder
// Provides: {"decode_vec"}
// Dependencies: {}
# [doc = " Decode a PEM document according to RFC 7468's \"Strict\" grammar, returning"] # [doc = " the result as a [`Vec`] upon success."] # [cfg (feature = "alloc")] pub fn decode_vec (pem : & [u8]) -> Result < (& str , Vec < u8 >) > { let mut decoder = Decoder :: new (pem) . map_err (| e | check_for_headers (pem , e)) ? ; let type_label = decoder . type_label () ; let mut buf = Vec :: new () ; decoder . decode_to_end (& mut buf) . map_err (| e | check_for_headers (pem , e)) ? ; Ok ((type_label , buf)) }
};
}
