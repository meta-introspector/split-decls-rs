// Generated macro for take_until_incomplete_s (function)
macro_rules! Depcrate_bytes_teststake_until_incomplete_s {
() => {
// Module: crate::bytes::tests
// Provides: {"take_until_incomplete_s"}
// Dependencies: {}
# [test] fn take_until_incomplete_s () { use crate :: bytes :: streaming :: take_until ; fn ys (i : & str) -> IResult < & str , & str > { take_until ("end") (i) } assert_eq ! (ys ("123en") , Err (Err :: Incomplete (Needed :: Unknown))) ; }
};
}
