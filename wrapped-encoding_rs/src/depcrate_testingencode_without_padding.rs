// Generated macro for encode_without_padding (function)
macro_rules! Depcrate_testingencode_without_padding {
() => {
// Module: crate::testing
// Provides: {"encode_without_padding"}
// Dependencies: {}
pub fn encode_without_padding (encoding : & 'static Encoding , string : & str , expect : & [u8]) { encode_from_utf8 (encoding , string , expect) ; encode_from_utf16 (encoding , & utf16_from_utf8 (string) [..] , expect) ; encode_to_vec (encoding , string , expect) ; }
};
}
