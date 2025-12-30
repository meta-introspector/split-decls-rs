// Generated macro for take_till (function)
macro_rules! Depcrate_bytes_teststake_till {
() => {
// Module: crate::bytes::tests
// Provides: {"take_till"}
// Dependencies: {}
# [test] fn take_till () { use crate :: bytes :: streaming :: take_till ; fn f (i : & [u8]) -> IResult < & [u8] , & [u8] > { take_till (AsChar :: is_alpha) (i) } let a = b"" ; let b = b"abcd" ; let c = b"123abcd" ; let d = b"123" ; assert_eq ! (f (& a [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (f (& b [..]) , Ok ((& b"abcd" [..] , & b"" [..]))) ; assert_eq ! (f (& c [..]) , Ok ((& b"abcd" [..] , & b"123" [..]))) ; assert_eq ! (f (& d [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; }
};
}
