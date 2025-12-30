// Generated macro for take_while1 (function)
macro_rules! Depcrate_bytes_teststake_while1 {
() => {
// Module: crate::bytes::tests
// Provides: {"take_while1"}
// Dependencies: {}
# [test] fn take_while1 () { use crate :: bytes :: streaming :: take_while1 ; fn f (i : & [u8]) -> IResult < & [u8] , & [u8] > { take_while1 (AsChar :: is_alpha) (i) } let a = b"" ; let b = b"abcd" ; let c = b"abcd123" ; let d = b"123" ; assert_eq ! (f (& a [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (f (& b [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (f (& c [..]) , Ok ((& b"123" [..] , & b [..]))) ; assert_eq ! (f (& d [..]) , Err (Err :: Error (error_position ! (& d [..] , ErrorKind :: TakeWhile1)))) ; }
};
}
