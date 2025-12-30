// Generated macro for f32_approx_eq_ratio_test2 (function)
macro_rules! Depcrate_ratiof32_approx_eq_ratio_test2 {
() => {
// Module: crate::ratio
// Provides: {"f32_approx_eq_ratio_test2"}
// Dependencies: {}
# [test] fn f32_approx_eq_ratio_test2 () { let x : f32 = 0.00000000001_f32 ; let y : f32 = 0.00000000005_f32 ; assert ! (x . approx_eq_ratio (& y , 0.81)) ; assert ! (y . approx_ne_ratio (& x , 0.79)) ; }
};
}
