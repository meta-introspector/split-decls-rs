// Generated macro for test_len (function)
macro_rules! Depcrate_tests_arraytest_len {
() => {
// Module: crate::tests::array
// Provides: {"test_len"}
// Dependencies: {}
# [test] fn test_len () { let empty_array = NSArray :: < NSObject > :: new () ; assert_eq ! (empty_array . len () , 0) ; let array = sample_array (4) ; assert_eq ! (array . len () , 4) ; }
};
}
