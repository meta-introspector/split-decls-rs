// Generated macro for test_field_invert (macro)
macro_rules! Depcrate_devtest_field_invert {
() => {
// Module: crate::dev
// Provides: {"test_field_invert"}
// Dependencies: {}
# [doc = " Implement field element inversion tests."] # [macro_export] macro_rules ! test_field_invert { ($ fe : tt) => { # [test] fn invert () { let one = $ fe :: ONE ; assert_eq ! (one . invert () . unwrap () , one) ; let three = one + & one + & one ; let inv_three = three . invert () . unwrap () ; assert_eq ! (three * & inv_three , one) ; let minus_three = - three ; let inv_minus_three = minus_three . invert () . unwrap () ; assert_eq ! (inv_minus_three , - inv_three) ; assert_eq ! (three * & inv_minus_three , - one) ; } } ; }
};
}
