// Generated macro for f64_approx_eq_test6 (function)
macro_rules! Depcrate_eqf64_approx_eq_test6 {
() => {
// Module: crate::eq
// Provides: {"f64_approx_eq_test6"}
// Dependencies: {}
# [test] fn f64_approx_eq_test6 () { let x : f64 = 1000000_f64 ; let y : f64 = 1000000.0000000003_f64 ; assert ! (x != y) ; assert ! (x . approx_eq (y , (0.0 , 3)) == true) ; }
};
}
