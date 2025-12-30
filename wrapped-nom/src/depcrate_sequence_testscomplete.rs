// Generated macro for complete (function)
macro_rules! Depcrate_sequence_testscomplete {
() => {
// Module: crate::sequence::tests
// Provides: {"complete"}
// Dependencies: {}
# [test] fn complete () { use crate :: bytes :: complete :: tag ; fn err_test (i : & [u8]) -> IResult < & [u8] , & [u8] > { let (i , _) = tag ("ijkl") (i) ? ; tag ("mnop") (i) } let a = & b"ijklmn" [..] ; let res_a = err_test (a) ; assert_eq ! (res_a , Err (Err :: Error (error_position ! (& b"mn" [..] , ErrorKind :: Tag)))) ; }
};
}
