// Generated macro for test_alloc_uninitialized_bools (function)
macro_rules! Depcrate_testtest_alloc_uninitialized_bools {
() => {
// Module: crate::test
// Provides: {"test_alloc_uninitialized_bools"}
// Dependencies: {}
# [doc = " Test with bools."] # [doc = ""] # [doc = " Bools, unlike integers, have invalid bit patterns. Therefore, ever having an uninitialized bool"] # [doc = " is insta-UB. Make sure miri doesn't find any such thing."] # [test] fn test_alloc_uninitialized_bools () { const LEN : usize = 20 ; unsafe { let arena : Arena < bool > = Arena :: with_capacity (2) ; let slice = arena . alloc_uninitialized (LEN) ; for elem in slice . iter_mut () { ptr :: write (elem . as_mut_ptr () , true) ; } let slice : & mut [bool] = mem :: transmute (slice) ; assert_eq ! (& [true ; LEN] , slice) ; } }
};
}
