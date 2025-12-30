// Generated macro for take_while_m_n (function)
macro_rules! Depcrate_bytes_teststake_while_m_n {
() => {
// Module: crate::bytes::tests
// Provides: {"take_while_m_n"}
// Dependencies: {}
# [test] fn take_while_m_n () { use crate :: bytes :: streaming :: take_while_m_n ; fn x (i : & [u8]) -> IResult < & [u8] , & [u8] > { take_while_m_n (2 , 4 , AsChar :: is_alpha) (i) } let a = b"" ; let b = b"a" ; let c = b"abc" ; let d = b"abc123" ; let e = b"abcde" ; let f = b"123" ; assert_eq ! (x (& a [..]) , Err (Err :: Incomplete (Needed :: new (2)))) ; assert_eq ! (x (& b [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (x (& c [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (x (& d [..]) , Ok ((& b"123" [..] , & b"abc" [..]))) ; assert_eq ! (x (& e [..]) , Ok ((& b"e" [..] , & b"abcd" [..]))) ; assert_eq ! (x (& f [..]) , Err (Err :: Error (error_position ! (& f [..] , ErrorKind :: TakeWhileMN)))) ; }
};
}
