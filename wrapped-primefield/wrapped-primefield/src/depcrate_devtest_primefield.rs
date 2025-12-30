// Generated macro for test_primefield (macro)
macro_rules! Depcrate_devtest_primefield {
() => {
// Module: crate::dev
// Provides: {"test_primefield"}
// Dependencies: {}
# [doc = " Implement all tests for a type which impls the `PrimeField` trait."] # [macro_export] macro_rules ! test_primefield { ($ fe : tt , $ uint : ident) => { $ crate :: test_primefield_constants ! ($ fe , $ uint) ; $ crate :: test_field_identity ! ($ fe) ; $ crate :: test_field_invert ! ($ fe) ; $ crate :: test_field_sqrt ! ($ fe) ; } ; }
};
}
