// Generated macro for f32_approx_eq_test4 (function)
macro_rules! Depcrate_eqf32_approx_eq_test4 {
() => {
// Module: crate::eq
// Provides: {"f32_approx_eq_test4"}
// Dependencies: {}
# [test] fn f32_approx_eq_test4 () { let f : f32 = 0.00001_f32 ; let g : f32 = 0.00000000000000001_f32 ; assert ! (f . approx_eq (g , (f32 :: EPSILON , 0)) == false) ; }
};
}
