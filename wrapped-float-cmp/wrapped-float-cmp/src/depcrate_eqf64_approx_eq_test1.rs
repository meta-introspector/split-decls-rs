// Generated macro for f64_approx_eq_test1 (function)
macro_rules! Depcrate_eqf64_approx_eq_test1 {
() => {
// Module: crate::eq
// Provides: {"f64_approx_eq_test1"}
// Dependencies: {}
# [test] fn f64_approx_eq_test1 () { let f : f64 = 0.0_f64 ; let g : f64 = - 0.0000000000000005551115123125783_f64 ; assert ! (f != g) ; assert ! (f . approx_eq (g , (3.0 * f64 :: EPSILON , 0)) == true) ; }
};
}
