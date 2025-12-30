// Generated macro for test_atomic_load_store (macro)
macro_rules! Depcrate_tests_helpertest_atomic_load_store {
() => {
// Module: crate::tests::helper
// Provides: {"test_atomic_load_store"}
// Dependencies: {}
macro_rules ! test_atomic_load_store { ($ ty : ident) => { paste :: paste ! { # [allow (clippy :: alloc_instead_of_core , clippy :: arithmetic_side_effects , clippy :: std_instead_of_alloc , clippy :: std_instead_of_core , clippy :: undocumented_unsafe_blocks ,)] mod [< test_atomic_ $ ty >] { __test_atomic ! (load_store , $ ty) ; } } } ; }
};
}
