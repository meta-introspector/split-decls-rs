// Generated macro for f32_approx_eq_ulps_test1 (function)
macro_rules! Depcrate_ulps_eqf32_approx_eq_ulps_test1 {
() => {
// Module: crate::ulps_eq
// Provides: {"f32_approx_eq_ulps_test1"}
// Dependencies: {}
# [test] fn f32_approx_eq_ulps_test1 () { let f : f32 = 0.1_f32 ; let mut sum : f32 = 0.0_f32 ; for _ in 0_isize .. 10_isize { sum += f ; } let product : f32 = f * 10.0_f32 ; assert ! (sum != product) ; assert ! (sum . approx_eq_ulps (& product , 1) == true) ; assert ! (sum . approx_eq_ulps (& product , 0) == false) ; }
};
}
