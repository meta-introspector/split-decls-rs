// Generated macro for test_primefield_constants (macro)
macro_rules! Depcrate_devtest_primefield_constants {
() => {
// Module: crate::dev
// Provides: {"test_primefield_constants"}
// Dependencies: {}
# [doc = " Implement tests for constants defined by the `PrimeField` trait."] # [macro_export] macro_rules ! test_primefield_constants { ($ fe : tt , $ uint : ident) => { use $ crate :: ff :: PrimeField as _ ; const T : $ uint = $ crate :: compute_t (&$ uint :: from_be_hex ($ fe :: MODULUS)) ; # [test] fn two_inv_constant () { assert_eq ! ($ fe :: from (2u32) * $ fe :: TWO_INV , $ fe :: ONE) ; } # [test] fn root_of_unity_constant () { assert_eq ! ($ fe :: ROOT_OF_UNITY . sqn_vartime ($ fe :: S as usize) , $ fe :: ONE) ; assert_eq ! ($ fe :: MULTIPLICATIVE_GENERATOR . pow_vartime (& T) , $ fe :: ROOT_OF_UNITY) } # [test] fn root_of_unity_inv_constant () { assert_eq ! ($ fe :: ROOT_OF_UNITY * $ fe :: ROOT_OF_UNITY_INV , $ fe :: ONE) ; } # [test] fn delta_constant () { assert_eq ! ($ fe :: DELTA . pow_vartime (& T) , $ fe :: ONE) ; } } ; }
};
}
