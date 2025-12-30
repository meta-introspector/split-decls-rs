// Generated macro for test_verify_alloc (function)
macro_rules! Depcrate_combinator_teststest_verify_alloc {
() => {
// Module: crate::combinator::tests
// Provides: {"test_verify_alloc"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] fn test_verify_alloc () { use crate :: bytes :: complete :: take ; let mut parser1 = verify (map (take (3u8) , | s : & [u8] | s . to_vec ()) , | s : & [u8] | { s == & b"abc" [..] }) ; assert_eq ! (parser1 . parse (& b"abcd" [..]) , Ok ((& b"d" [..] , b"abc" . to_vec ()))) ; assert_eq ! (parser1 . parse (& b"defg" [..]) , Err (Err :: Error ((& b"defg" [..] , ErrorKind :: Verify)))) ; }
};
}
