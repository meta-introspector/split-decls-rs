// Generated macro for decode_to_string (function)
macro_rules! Depcrate_testingdecode_to_string {
() => {
// Module: crate::testing
// Provides: {"decode_to_string"}
// Dependencies: {}
pub fn decode_to_string (encoding : & 'static Encoding , bytes : & [u8] , expect : & str) { let (cow , _ , _) = encoding . decode (bytes) ; assert_eq ! (& cow [..] , expect) ; }
};
}
