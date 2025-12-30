// Generated macro for encode_to_vec (function)
macro_rules! Depcrate_testingencode_to_vec {
() => {
// Module: crate::testing
// Provides: {"encode_to_vec"}
// Dependencies: {}
pub fn encode_to_vec (encoding : & 'static Encoding , string : & str , expect : & [u8]) { let (cow , _ , _) = encoding . encode (string) ; assert_eq ! (& cow [..] , expect) ; }
};
}
