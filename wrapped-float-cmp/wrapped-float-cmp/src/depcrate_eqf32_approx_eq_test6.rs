// Generated macro for f32_approx_eq_test6 (function)
macro_rules! Depcrate_eqf32_approx_eq_test6 {
() => {
// Module: crate::eq
// Provides: {"f32_approx_eq_test6"}
// Dependencies: {}
# [test] fn f32_approx_eq_test6 () { let x : f32 = 1000000_f32 ; let y : f32 = 1000000.1_f32 ; assert ! (x != y) ; assert ! (x . approx_eq (y , (0.0 , 2)) == true) ; assert ! (x . approx_eq (y , (1000.0 * f32 :: EPSILON , 0)) == false) ; }
};
}
