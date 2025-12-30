// Generated macro for test_verify_ref (function)
macro_rules! Depcrate_combinator_teststest_verify_ref {
() => {
// Module: crate::combinator::tests
// Provides: {"test_verify_ref"}
// Dependencies: {}
# [test] # [allow (unused)] fn test_verify_ref () { use crate :: bytes :: complete :: take ; let mut parser1 = verify (take (3u8) , | s : & [u8] | s == & b"abc" [..]) ; assert_eq ! (parser1 . parse (& b"abcd" [..]) , Ok ((& b"d" [..] , & b"abc" [..]))) ; assert_eq ! (parser1 . parse (& b"defg" [..]) , Err (Err :: Error ((& b"defg" [..] , ErrorKind :: Verify)))) ; fn parser2 (i : & [u8]) -> IResult < & [u8] , u32 > { verify (crate :: number :: streaming :: be_u32 , | val : & u32 | * val < 3) . parse (i) } }
};
}
