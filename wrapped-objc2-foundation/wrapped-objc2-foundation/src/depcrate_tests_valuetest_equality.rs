// Generated macro for test_equality (function)
macro_rules! Depcrate_tests_valuetest_equality {
() => {
// Module: crate::tests::value
// Provides: {"test_equality"}
// Dependencies: {}
# [test] fn test_equality () { let val1 = NSValue :: new (123u32) ; let val2 = NSValue :: new (123u32) ; assert_eq ! (val1 , val1) ; assert_eq ! (val1 , val2) ; let val3 = NSValue :: new (456u32) ; assert_ne ! (val1 , val3) ; }
};
}
