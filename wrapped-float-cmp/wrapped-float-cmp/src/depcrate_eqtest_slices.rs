// Generated macro for test_slices (function)
macro_rules! Depcrate_eqtest_slices {
() => {
// Module: crate::eq
// Provides: {"test_slices"}
// Dependencies: {}
# [test] fn test_slices () { assert ! ([1.33 , 2.4 , 2.5] . approx_eq (& [1.33 , 2.4 , 2.5] , (0.0 , 0_i64))) ; assert ! (! [1.33 , 2.4 , 2.6] . approx_eq (& [1.33 , 2.4 , 2.5] , (0.0 , 0_i64))) ; assert ! (! [1.33 , 2.4] . approx_eq (& [1.33 , 2.4 , 2.5] , (0.0 , 0_i64))) ; assert ! (! [1.33 , 2.4 , 2.5] . approx_eq (& [1.33 , 2.4] , (0.0 , 0_i64))) ; }
};
}
