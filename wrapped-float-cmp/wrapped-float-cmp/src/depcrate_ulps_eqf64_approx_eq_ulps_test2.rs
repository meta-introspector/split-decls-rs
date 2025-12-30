// Generated macro for f64_approx_eq_ulps_test2 (function)
macro_rules! Depcrate_ulps_eqf64_approx_eq_ulps_test2 {
() => {
// Module: crate::ulps_eq
// Provides: {"f64_approx_eq_ulps_test2"}
// Dependencies: {}
# [test] fn f64_approx_eq_ulps_test2 () { let x : f64 = 1000000_f64 ; let y : f64 = 1000000.0000000003_f64 ; assert ! (x != y) ; assert ! (x . approx_eq_ulps (& y , 3) == true) ; assert ! (x . approx_eq_ulps (& y , 2) == false) ; }
};
}
