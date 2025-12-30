// Generated macro for test_debug (function)
macro_rules! Depcrate_tests_arraytest_debug {
() => {
// Module: crate::tests::array
// Provides: {"test_debug"}
// Dependencies: {}
# [test] fn test_debug () { let obj = sample_number_array (0) ; assert_eq ! (format ! ("{obj:?}") , "[]") ; let obj = sample_number_array (3) ; assert_eq ! (format ! ("{obj:?}") , "[0, 1, 2]") ; }
};
}
