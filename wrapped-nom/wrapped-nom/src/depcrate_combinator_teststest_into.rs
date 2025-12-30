// Generated macro for test_into (function)
macro_rules! Depcrate_combinator_teststest_into {
() => {
// Module: crate::combinator::tests
// Provides: {"test_into"}
// Dependencies: {}
# [test] # [cfg (feature = "std")] fn test_into () { use crate :: bytes :: complete :: take ; use crate :: { error :: { Error , ParseError } , Err , } ; let mut parser = into (take :: < _ , _ , Error < _ > > (3u8)) ; let result : IResult < & [u8] , Vec < u8 > > = parser . parse (& b"abcdefg" [..]) ; assert_eq ! (result , Ok ((& b"defg" [..] , vec ! [97 , 98 , 99]))) ; }
};
}
