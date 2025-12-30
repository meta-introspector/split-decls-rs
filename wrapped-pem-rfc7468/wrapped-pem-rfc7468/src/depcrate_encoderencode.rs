// Generated macro for encode (function)
macro_rules! Depcrate_encoderencode {
() => {
// Module: crate::encoder
// Provides: {"encode"}
// Dependencies: {}
# [doc = " Encode a PEM document according to RFC 7468's \"Strict\" grammar."] pub fn encode < 'o > (type_label : & str , line_ending : LineEnding , input : & [u8] , buf : & 'o mut [u8] ,) -> Result < & 'o str > { let mut encoder = Encoder :: new (type_label , line_ending , buf) ? ; encoder . encode (input) ? ; let encoded_len = encoder . finish () ? ; let output = & buf [.. encoded_len] ; debug_assert ! (str :: from_utf8 (output) . is_ok ()) ; if output . iter () . fold (0u8 , | acc , & byte | acc | (byte & 0x80)) == 0 { # [allow (unsafe_code)] Ok (unsafe { str :: from_utf8_unchecked (output) }) } else { Err (Error :: CharacterEncoding) } }
};
}
