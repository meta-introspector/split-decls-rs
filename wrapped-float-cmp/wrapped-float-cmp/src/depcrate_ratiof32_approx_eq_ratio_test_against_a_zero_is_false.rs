// Generated macro for f32_approx_eq_ratio_test_against_a_zero_is_false (function)
macro_rules! Depcrate_ratiof32_approx_eq_ratio_test_against_a_zero_is_false {
() => {
// Module: crate::ratio
// Provides: {"f32_approx_eq_ratio_test_against_a_zero_is_false"}
// Dependencies: {}
# [test] fn f32_approx_eq_ratio_test_against_a_zero_is_false () { let x : f32 = 0.0_f32 ; let y : f32 = 0.1_f32 ; assert ! (x . approx_eq_ratio (& y , 0.1) == false) ; assert ! (y . approx_eq_ratio (& x , 0.1) == false) ; }
};
}
