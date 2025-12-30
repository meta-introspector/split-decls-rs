// Generated macro for f32_approx_eq_test1 (function)
macro_rules! Depcrate_eqf32_approx_eq_test1 {
() => {
// Module: crate::eq
// Provides: {"f32_approx_eq_test1"}
// Dependencies: {}
# [test] fn f32_approx_eq_test1 () { let f : f32 = 0.0_f32 ; let g : f32 = - 0.0000000000000005551115123125783_f32 ; assert ! (f != g) ; assert ! (f . approx_eq (g , (f32 :: EPSILON , 0)) == true) ; }
};
}
