// Generated macro for f32_approx_eq_ulps_test2 (function)
macro_rules! Depcrate_ulps_eqf32_approx_eq_ulps_test2 {
() => {
// Module: crate::ulps_eq
// Provides: {"f32_approx_eq_ulps_test2"}
// Dependencies: {}
# [test] fn f32_approx_eq_ulps_test2 () { let x : f32 = 1000000_f32 ; let y : f32 = 1000000.1_f32 ; assert ! (x != y) ; assert ! (x . approx_eq_ulps (& y , 2) == true) ; assert ! (x . approx_eq_ulps (& y , 1) == false) ; }
};
}
