// Generated macro for decode_label (function)
macro_rules! Depcrate_decoderdecode_label {
() => {
// Module: crate::decoder
// Provides: {"decode_label"}
// Dependencies: {}
# [doc = " Decode the encapsulation boundaries of a PEM document according to RFC 7468's \"Strict\" grammar."] # [doc = ""] # [doc = " On success, returning the decoded label."] pub fn decode_label (pem : & [u8]) -> Result < & str > { Ok (Encapsulation :: try_from (pem) ? . label ()) }
};
}
