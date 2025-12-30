// Generated macro for test_debug (function)
macro_rules! Depcrate_tests_datatest_debug {
() => {
// Module: crate::tests::data
// Provides: {"test_debug"}
// Dependencies: {}
# [test] fn test_debug () { let bytes = [3 , 7 , 16 , 52 , 112 , 19] ; let data = NSData :: with_bytes (& bytes) ; assert_eq ! (format ! ("{data:?}") , "[3, 7, 16, 52, 112, 19]") ; }
};
}
