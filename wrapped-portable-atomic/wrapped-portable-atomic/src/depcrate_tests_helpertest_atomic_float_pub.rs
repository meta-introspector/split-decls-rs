// Generated macro for test_atomic_float_pub (macro)
macro_rules! Depcrate_tests_helpertest_atomic_float_pub {
() => {
// Module: crate::tests::helper
// Provides: {"test_atomic_float_pub"}
// Dependencies: {}
# [cfg (feature = "float")] macro_rules ! test_atomic_float_pub { ($ float_type : ident) => { paste :: paste ! { # [allow (clippy :: alloc_instead_of_core , clippy :: float_arithmetic , clippy :: std_instead_of_alloc , clippy :: std_instead_of_core , clippy :: undocumented_unsafe_blocks)] mod [< test_atomic_ $ float_type >] { use super ::*; __test_atomic_float_load_store ! ([< Atomic $ float_type : camel >] , $ float_type) ; __test_atomic_float ! ([< Atomic $ float_type : camel >] , $ float_type) ; __test_atomic_float_pub ! ([< Atomic $ float_type : camel >] , $ float_type) ; } } } ; }
};
}
