// Generated macro for is_not (function)
macro_rules! Depcrate_bytes_testsis_not {
() => {
// Module: crate::bytes::tests
// Provides: {"is_not"}
// Dependencies: {}
# [test] fn is_not () { use crate :: bytes :: streaming :: is_not ; fn a_or_b (i : & [u8]) -> IResult < & [u8] , & [u8] > { is_not ("ab") (i) } let a = & b"cdab" [..] ; assert_eq ! (a_or_b (a) , Ok ((& b"ab" [..] , & b"cd" [..]))) ; let b = & b"cbde" [..] ; assert_eq ! (a_or_b (b) , Ok ((& b"bde" [..] , & b"c" [..]))) ; let c = & b"abab" [..] ; assert_eq ! (a_or_b (c) , Err (Err :: Error (error_position ! (c , ErrorKind :: IsNot)))) ; let d = & b"cdefba" [..] ; assert_eq ! (a_or_b (d) , Ok ((& b"ba" [..] , & b"cdef" [..]))) ; let e = & b"e" [..] ; assert_eq ! (a_or_b (e) , Err (Err :: Incomplete (Needed :: new (1)))) ; }
};
}
