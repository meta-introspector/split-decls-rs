// Generated macro for f32_approx_eq_test5 (function)
macro_rules! Depcrate_eqf32_approx_eq_test5 {
() => {
// Module: crate::eq
// Provides: {"f32_approx_eq_test5"}
// Dependencies: {}
# [test] fn f32_approx_eq_test5 () { let f : f32 = 0.1_f32 ; let mut sum : f32 = 0.0_f32 ; for _ in 0_isize .. 10_isize { sum += f ; } let product : f32 = f * 10.0_f32 ; assert ! (sum != product) ; assert ! (sum . approx_eq (product , (f32 :: EPSILON , 1)) == true) ; assert ! (sum . approx_eq (product , F32Margin :: zero ()) == false) ; }
};
}
