// Generated macro for test_field_identity (macro)
macro_rules! Depcrate_devtest_field_identity {
() => {
// Module: crate::dev
// Provides: {"test_field_identity"}
// Dependencies: {}
# [doc = " Implement field element identity tests."] # [macro_export] macro_rules ! test_field_identity { ($ fe : tt) => { # [test] fn zero_is_additive_identity () { let zero = $ fe :: ZERO ; let one = $ fe :: ONE ; assert_eq ! (zero . add (& zero) , zero) ; assert_eq ! (one . add (& zero) , one) ; } # [test] fn one_is_multiplicative_identity () { let one = $ fe :: ONE ; assert_eq ! (one . multiply (& one) , one) ; } } ; }
};
}
