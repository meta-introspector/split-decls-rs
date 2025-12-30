// Generated macro for take_while (function)
macro_rules! Depcrate_bytes_teststake_while {
() => {
// Module: crate::bytes::tests
// Provides: {"take_while"}
// Dependencies: {}
# [test] fn take_while () { use crate :: bytes :: streaming :: take_while ; fn f (i : & [u8]) -> IResult < & [u8] , & [u8] > { take_while (AsChar :: is_alpha) (i) } let a = b"" ; let b = b"abcd" ; let c = b"abcd123" ; let d = b"123" ; assert_eq ! (f (& a [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (f (& b [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (f (& c [..]) , Ok ((& d [..] , & b [..]))) ; assert_eq ! (f (& d [..]) , Ok ((& d [..] , & a [..]))) ; }
};
}
