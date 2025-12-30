// Generated macro for test_atomic (macro)
macro_rules! Depcrate_tests_helpertest_atomic {
() => {
// Module: crate::tests::helper
// Provides: {"test_atomic"}
// Dependencies: {}
macro_rules ! test_atomic { ($ ty : ident) => { paste :: paste ! { # [allow (clippy :: alloc_instead_of_core , clippy :: arithmetic_side_effects , clippy :: std_instead_of_alloc , clippy :: std_instead_of_core , clippy :: undocumented_unsafe_blocks ,)] mod [< test_atomic_ $ ty >] { __test_atomic ! (load_store , $ ty) ; # [cfg (not (all (target_arch = "csky" , atomic_maybe_uninit_no_ldex_stex)))] __test_atomic ! (swap , $ ty) ; # [cfg (not (all (target_arch = "csky" , atomic_maybe_uninit_no_ldex_stex)))] # [cfg (not (all (target_arch = "x86" , atomic_maybe_uninit_no_cmpxchg)))] __test_atomic ! (cas , $ ty) ; } } } ; }
};
}
