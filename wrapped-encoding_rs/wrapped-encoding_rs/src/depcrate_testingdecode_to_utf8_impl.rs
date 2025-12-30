// Generated macro for decode_to_utf8_impl (function)
macro_rules! Depcrate_testingdecode_to_utf8_impl {
() => {
// Module: crate::testing
// Provides: {"decode_to_utf8_impl"}
// Dependencies: {}
pub fn decode_to_utf8_impl (encoding : & 'static Encoding , bytes : & [u8] , expect : & str , padding : usize ,) { for i in padding .. bytes . len () { let (head , tail) = bytes . split_at (i) ; decode_to_utf8_with_boundary (encoding , head , tail , expect) ; } }
};
}
