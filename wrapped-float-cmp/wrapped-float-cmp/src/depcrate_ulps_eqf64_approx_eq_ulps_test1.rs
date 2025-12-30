// Generated macro for f64_approx_eq_ulps_test1 (function)
macro_rules! Depcrate_ulps_eqf64_approx_eq_ulps_test1 {
() => {
// Module: crate::ulps_eq
// Provides: {"f64_approx_eq_ulps_test1"}
// Dependencies: {}
# [test] fn f64_approx_eq_ulps_test1 () { let f : f64 = 0.1_f64 ; let mut sum : f64 = 0.0_f64 ; for _ in 0_isize .. 10_isize { sum += f ; } let product : f64 = f * 10.0_f64 ; assert ! (sum != product) ; assert ! (sum . approx_eq_ulps (& product , 1) == true) ; assert ! (sum . approx_eq_ulps (& product , 0) == false) ; }
};
}
