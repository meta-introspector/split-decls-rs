// Generated macro for f64_approx_eq_ratio_test1 (function)
macro_rules! Depcrate_ratiof64_approx_eq_ratio_test1 {
() => {
// Module: crate::ratio
// Provides: {"f64_approx_eq_ratio_test1"}
// Dependencies: {}
# [test] fn f64_approx_eq_ratio_test1 () { let x : f64 = 0.000000004_f64 ; let y : f64 = 0.000000004001_f64 ; assert ! (x . approx_eq_ratio (& y , 0.00025)) ; assert ! (y . approx_eq_ratio (& x , 0.00025)) ; assert ! (x . approx_ne_ratio (& y , 0.00024)) ; assert ! (y . approx_ne_ratio (& x , 0.00024)) ; }
};
}
