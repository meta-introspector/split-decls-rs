// Generated macro for is_a (function)
macro_rules! Depcrate_bytes_testsis_a {
() => {
// Module: crate::bytes::tests
// Provides: {"is_a"}
// Dependencies: {}
# [test] fn is_a () { use crate :: bytes :: streaming :: is_a ; fn a_or_b (i : & [u8]) -> IResult < & [u8] , & [u8] > { is_a ("ab") (i) } let a = & b"abcd" [..] ; assert_eq ! (a_or_b (a) , Ok ((& b"cd" [..] , & b"ab" [..]))) ; let b = & b"bcde" [..] ; assert_eq ! (a_or_b (b) , Ok ((& b"cde" [..] , & b"b" [..]))) ; let c = & b"cdef" [..] ; assert_eq ! (a_or_b (c) , Err (Err :: Error (error_position ! (c , ErrorKind :: IsA)))) ; let d = & b"bacdef" [..] ; assert_eq ! (a_or_b (d) , Ok ((& b"cdef" [..] , & b"ba" [..]))) ; }
};
}
