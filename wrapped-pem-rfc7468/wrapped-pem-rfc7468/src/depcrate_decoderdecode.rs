// Generated macro for decode (function)
macro_rules! Depcrate_decoderdecode {
() => {
// Module: crate::decoder
// Provides: {"decode"}
// Dependencies: {}
# [doc = " Decode a PEM document according to RFC 7468's \"Strict\" grammar."] # [doc = ""] # [doc = " On success, writes the decoded document into the provided buffer, returning"] # [doc = " the decoded label and the portion of the provided buffer containing the"] # [doc = " decoded message."] pub fn decode < 'i , 'o > (pem : & 'i [u8] , buf : & 'o mut [u8]) -> Result < (& 'i str , & 'o [u8]) > { let mut decoder = Decoder :: new (pem) . map_err (| e | check_for_headers (pem , e)) ? ; let type_label = decoder . type_label () ; let buf = buf . get_mut (.. decoder . remaining_len ()) . ok_or (Error :: Length) ? ; let decoded = decoder . decode (buf) . map_err (| e | check_for_headers (pem , e)) ? ; if decoder . base64 . is_finished () { Ok ((type_label , decoded)) } else { Err (Error :: Length) } }
};
}
