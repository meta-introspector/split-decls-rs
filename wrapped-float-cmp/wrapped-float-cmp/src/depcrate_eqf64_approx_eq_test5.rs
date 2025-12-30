// Generated macro for f64_approx_eq_test5 (function)
macro_rules! Depcrate_eqf64_approx_eq_test5 {
() => {
// Module: crate::eq
// Provides: {"f64_approx_eq_test5"}
// Dependencies: {}
# [test] fn f64_approx_eq_test5 () { let f : f64 = 0.1_f64 ; let mut sum : f64 = 0.0_f64 ; for _ in 0_isize .. 10_isize { sum += f ; } let product : f64 = f * 10.0_f64 ; assert ! (sum != product) ; assert ! (sum . approx_eq (product , (f64 :: EPSILON , 0)) == true) ; assert ! (sum . approx_eq (product , (0.0 , 1)) == true) ; }
};
}
