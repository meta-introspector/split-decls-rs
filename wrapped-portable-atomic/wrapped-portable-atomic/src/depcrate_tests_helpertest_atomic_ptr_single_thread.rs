// Generated macro for test_atomic_ptr_single_thread (macro)
macro_rules! Depcrate_tests_helpertest_atomic_ptr_single_thread {
() => {
// Module: crate::tests::helper
// Provides: {"test_atomic_ptr_single_thread"}
// Dependencies: {}
macro_rules ! test_atomic_ptr_single_thread { () => { # [allow (clippy :: alloc_instead_of_core , clippy :: std_instead_of_alloc , clippy :: std_instead_of_core , clippy :: undocumented_unsafe_blocks)] mod test_atomic_ptr { use super ::*; __test_atomic_ptr_load_store ! (AtomicPtr < u8 >, single_thread) ; __test_atomic_ptr ! (AtomicPtr < u8 >, single_thread) ; } } ; }
};
}
