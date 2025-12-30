// Generated macro for f32_approx_eq_ratio_test1 (function)
macro_rules! Depcrate_ratiof32_approx_eq_ratio_test1 {
() => {
// Module: crate::ratio
// Provides: {"f32_approx_eq_ratio_test1"}
// Dependencies: {}
# [test] fn f32_approx_eq_ratio_test1 () { let x : f32 = 0.00004_f32 ; let y : f32 = 0.00004001_f32 ; assert ! (x . approx_eq_ratio (& y , 0.00025)) ; assert ! (y . approx_eq_ratio (& x , 0.00025)) ; assert ! (x . approx_ne_ratio (& y , 0.00024)) ; assert ! (y . approx_ne_ratio (& x , 0.00024)) ; }
};
}
