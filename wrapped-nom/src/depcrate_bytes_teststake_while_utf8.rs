// Generated macro for take_while_utf8 (function)
macro_rules! Depcrate_bytes_teststake_while_utf8 {
() => {
// Module: crate::bytes::tests
// Provides: {"take_while_utf8"}
// Dependencies: {}
# [test] fn take_while_utf8 () { use crate :: bytes :: streaming :: take_while ; fn f (i : & str) -> IResult < & str , & str > { take_while (| c | c != '點') (i) } assert_eq ! (f ("") , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (f ("abcd") , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (f ("abcd點") , Ok (("點" , "abcd"))) ; assert_eq ! (f ("abcd點a") , Ok (("點a" , "abcd"))) ; fn g (i : & str) -> IResult < & str , & str > { take_while (| c | c == '點') (i) } assert_eq ! (g ("") , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (g ("點abcd") , Ok (("abcd" , "點"))) ; assert_eq ! (g ("點點點a") , Ok (("a" , "點點點"))) ; }
};
}
