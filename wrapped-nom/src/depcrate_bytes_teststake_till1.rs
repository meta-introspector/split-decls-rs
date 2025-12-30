// Generated macro for take_till1 (function)
macro_rules! Depcrate_bytes_teststake_till1 {
() => {
// Module: crate::bytes::tests
// Provides: {"take_till1"}
// Dependencies: {}
# [test] fn take_till1 () { use crate :: bytes :: streaming :: take_till1 ; fn f (i : & [u8]) -> IResult < & [u8] , & [u8] > { take_till1 (AsChar :: is_alpha) (i) } let a = b"" ; let b = b"abcd" ; let c = b"123abcd" ; let d = b"123" ; assert_eq ! (f (& a [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (f (& b [..]) , Err (Err :: Error (error_position ! (& b [..] , ErrorKind :: TakeTill1)))) ; assert_eq ! (f (& c [..]) , Ok ((& b"abcd" [..] , & b"123" [..]))) ; assert_eq ! (f (& d [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; }
};
}
