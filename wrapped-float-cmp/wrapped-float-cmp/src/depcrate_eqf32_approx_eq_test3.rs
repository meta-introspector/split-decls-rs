// Generated macro for f32_approx_eq_test3 (function)
macro_rules! Depcrate_eqf32_approx_eq_test3 {
() => {
// Module: crate::eq
// Provides: {"f32_approx_eq_test3"}
// Dependencies: {}
# [test] fn f32_approx_eq_test3 () { let f : f32 = 0.0_f32 ; let g : f32 = 0.00000000000000001_f32 ; assert ! (f . approx_eq (g , (f32 :: EPSILON , 0)) == true) ; }
};
}
