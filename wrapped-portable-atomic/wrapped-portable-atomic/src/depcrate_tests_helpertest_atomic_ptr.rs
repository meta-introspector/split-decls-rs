// Generated macro for test_atomic_ptr (macro)
macro_rules! Depcrate_tests_helpertest_atomic_ptr {
() => {
// Module: crate::tests::helper
// Provides: {"test_atomic_ptr"}
// Dependencies: {}
macro_rules ! test_atomic_ptr { () => { # [allow (clippy :: alloc_instead_of_core , clippy :: std_instead_of_alloc , clippy :: std_instead_of_core , clippy :: undocumented_unsafe_blocks)] # [allow (unstable_name_collisions)] mod test_atomic_ptr { use super ::*; __test_atomic_ptr_load_store ! (AtomicPtr < u8 >) ; __test_atomic_ptr ! (AtomicPtr < u8 >) ; } } ; }
};
}
