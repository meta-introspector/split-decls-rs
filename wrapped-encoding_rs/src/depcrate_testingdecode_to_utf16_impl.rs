// Generated macro for decode_to_utf16_impl (function)
macro_rules! Depcrate_testingdecode_to_utf16_impl {
() => {
// Module: crate::testing
// Provides: {"decode_to_utf16_impl"}
// Dependencies: {}
pub fn decode_to_utf16_impl (encoding : & 'static Encoding , bytes : & [u8] , expect : & [u16] , padding : usize ,) { for i in padding .. bytes . len () { let (head , tail) = bytes . split_at (i) ; decode_to_utf16_with_boundary (encoding , head , tail , expect) ; } }
};
}
