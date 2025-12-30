// Generated macro for f64_approx_eq_ratio_test2 (function)
macro_rules! Depcrate_ratiof64_approx_eq_ratio_test2 {
() => {
// Module: crate::ratio
// Provides: {"f64_approx_eq_ratio_test2"}
// Dependencies: {}
# [test] fn f64_approx_eq_ratio_test2 () { let x : f64 = 0.0000000000000001_f64 ; let y : f64 = 0.0000000000000005_f64 ; assert ! (x . approx_eq_ratio (& y , 0.81)) ; assert ! (y . approx_ne_ratio (& x , 0.79)) ; }
};
}
