// Generated macro for test_equality (function)
macro_rules! Depcrate_tests_arraytest_equality {
() => {
// Module: crate::tests::array
// Provides: {"test_equality"}
// Dependencies: {}
# [test] fn test_equality () { let array1 = sample_array (3) ; let array2 = sample_array (3) ; assert_ne ! (array1 , array2) ; let array1 = sample_number_array (3) ; let array2 = sample_number_array (3) ; assert_eq ! (array1 , array2) ; let array1 = sample_number_array (3) ; let array2 = sample_number_array (4) ; assert_ne ! (array1 , array2) ; }
};
}
