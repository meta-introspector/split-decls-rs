// Generated macro for __test_atomic_pub_common (macro)
macro_rules! Depcrate_tests_helper__test_atomic_pub_common {
() => {
// Module: crate::tests::helper
// Provides: {"__test_atomic_pub_common"}
// Dependencies: {}
macro_rules ! __test_atomic_pub_common { ($ atomic_type : ty , $ value_type : ty) => { # [test] fn is_always_lock_free () { assert_eq ! (<$ atomic_type >:: IS_ALWAYS_LOCK_FREE , <$ atomic_type >:: is_always_lock_free ()) ; } # [test] fn assert_ref_unwind_safe () { # [cfg (not (all (portable_atomic_no_core_unwind_safe , not (feature = "std"))))] static_assertions :: assert_impl_all ! ($ atomic_type : std :: panic :: RefUnwindSafe) ; # [cfg (all (portable_atomic_no_core_unwind_safe , not (feature = "std")))] static_assertions :: assert_not_impl_all ! ($ atomic_type : std :: panic :: RefUnwindSafe) ; } } ; }
};
}
