// Generated macro for fail_test (function)
macro_rules! Depcrate_combinator_testsfail_test {
() => {
// Module: crate::combinator::tests
// Provides: {"fail_test"}
// Dependencies: {}
# [test] fn fail_test () { let a = "string" ; let b = "another string" ; assert_eq ! (fail ::< _ , & str , _ > () . parse (a) , Err (Err :: Error ((a , ErrorKind :: Fail)))) ; assert_eq ! (fail ::< _ , & str , _ > () . parse (b) , Err (Err :: Error ((b , ErrorKind :: Fail)))) ; }
};
}
