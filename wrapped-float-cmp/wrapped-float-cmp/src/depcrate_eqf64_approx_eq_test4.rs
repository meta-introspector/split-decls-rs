// Generated macro for f64_approx_eq_test4 (function)
macro_rules! Depcrate_eqf64_approx_eq_test4 {
() => {
// Module: crate::eq
// Provides: {"f64_approx_eq_test4"}
// Dependencies: {}
# [test] fn f64_approx_eq_test4 () { let f : f64 = 0.00001_f64 ; let g : f64 = 0.00000000000000001_f64 ; assert ! (f . approx_eq (g , (f64 :: EPSILON , 0)) == false) ; }
};
}
