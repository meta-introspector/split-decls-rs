// Generated macro for not_test (function)
macro_rules! Depcrate_combinator_testsnot_test {
() => {
// Module: crate::combinator::tests
// Provides: {"not_test"}
// Dependencies: {}
# [test] fn not_test () { fn not_aaa (i : & [u8]) -> IResult < & [u8] , () > { not (tag ("aaa")) . parse (i) } assert_eq ! (not_aaa (& b"aaa" [..]) , Err (Err :: Error (error_position ! (& b"aaa" [..] , ErrorKind :: Not)))) ; assert_eq ! (not_aaa (& b"aa" [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (not_aaa (& b"abcd" [..]) , Ok ((& b"abcd" [..] , ()))) ; }
};
}
