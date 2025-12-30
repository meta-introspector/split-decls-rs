// Generated macro for length_bytes (function)
macro_rules! Depcrate_bytes_testslength_bytes {
() => {
// Module: crate::bytes::tests
// Provides: {"length_bytes"}
// Dependencies: {}
# [test] fn length_bytes () { use crate :: { bytes :: streaming :: tag , multi :: length_data , number :: streaming :: le_u8 } ; fn x (i : & [u8]) -> IResult < & [u8] , & [u8] > { length_data (le_u8) . parse (i) } assert_eq ! (x (b"\x02..>>") , Ok ((& b">>" [..] , & b".." [..]))) ; assert_eq ! (x (b"\x02..") , Ok ((& [] [..] , & b".." [..]))) ; assert_eq ! (x (b"\x02.") , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (x (b"\x02") , Err (Err :: Incomplete (Needed :: new (2)))) ; fn y (i : & [u8]) -> IResult < & [u8] , & [u8] > { let (i , _) = tag ("magic") (i) ? ; length_data (le_u8) . parse (i) } assert_eq ! (y (b"magic\x02..>>") , Ok ((& b">>" [..] , & b".." [..]))) ; assert_eq ! (y (b"magic\x02..") , Ok ((& [] [..] , & b".." [..]))) ; assert_eq ! (y (b"magic\x02.") , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (y (b"magic\x02") , Err (Err :: Incomplete (Needed :: new (2)))) ; }
};
}
