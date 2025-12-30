// Generated macro for __test_atomic_common (macro)
macro_rules! Depcrate_tests_helper__test_atomic_common {
() => {
// Module: crate::tests::helper
// Provides: {"__test_atomic_common"}
// Dependencies: {}
macro_rules ! __test_atomic_common { ($ atomic_type : ty , $ value_type : ty) => { use std :: mem ; # [test] fn assert_auto_traits () { fn _assert < T : Send + Sync + Unpin + std :: panic :: UnwindSafe > () { } _assert ::<$ atomic_type > () ; } # [test] fn alignment () { assert_eq ! (mem :: align_of ::<$ atomic_type > () , mem :: size_of ::<$ atomic_type > ()) ; assert_eq ! (mem :: size_of ::<$ atomic_type > () , mem :: size_of ::<$ value_type > ()) ; } # [test] fn is_lock_free () { const IS_ALWAYS_LOCK_FREE : bool = <$ atomic_type >:: IS_ALWAYS_LOCK_FREE ; assert_eq ! (IS_ALWAYS_LOCK_FREE , <$ atomic_type >:: IS_ALWAYS_LOCK_FREE) ; let is_lock_free = <$ atomic_type >:: is_lock_free () ; if IS_ALWAYS_LOCK_FREE { assert ! (is_lock_free) ; } } } ; }
};
}
