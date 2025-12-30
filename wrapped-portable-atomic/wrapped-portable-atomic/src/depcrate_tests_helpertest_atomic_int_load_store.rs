// Generated macro for test_atomic_int_load_store (macro)
macro_rules! Depcrate_tests_helpertest_atomic_int_load_store {
() => {
// Module: crate::tests::helper
// Provides: {"test_atomic_int_load_store"}
// Dependencies: {}
macro_rules ! test_atomic_int_load_store { ($ int_type : ident) => { paste :: paste ! { # [allow (clippy :: alloc_instead_of_core , clippy :: std_instead_of_alloc , clippy :: std_instead_of_core , clippy :: undocumented_unsafe_blocks)] mod [< test_atomic_ $ int_type >] { use super ::*; __test_atomic_int_load_store ! ([< Atomic $ int_type : camel >] , $ int_type) ; } } } ; }
};
}
