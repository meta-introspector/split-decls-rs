// Generated macro for test_atomic_int_pub (macro)
macro_rules! Depcrate_tests_helpertest_atomic_int_pub {
() => {
// Module: crate::tests::helper
// Provides: {"test_atomic_int_pub"}
// Dependencies: {}
macro_rules ! test_atomic_int_pub { ($ int_type : ident) => { paste :: paste ! { # [allow (clippy :: alloc_instead_of_core , clippy :: std_instead_of_alloc , clippy :: std_instead_of_core , clippy :: undocumented_unsafe_blocks)] mod [< test_atomic_ $ int_type >] { use super ::*; __test_atomic_int_load_store ! ([< Atomic $ int_type : camel >] , $ int_type) ; __test_atomic_int ! ([< Atomic $ int_type : camel >] , $ int_type) ; __test_atomic_int_pub ! ([< Atomic $ int_type : camel >] , $ int_type) ; } } } ; }
};
}
