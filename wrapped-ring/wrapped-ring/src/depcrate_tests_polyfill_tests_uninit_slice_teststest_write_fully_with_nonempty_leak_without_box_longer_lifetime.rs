// Generated macro for test_write_fully_with_nonempty_leak_without_box_longer_lifetime (function)
macro_rules! Depcrate_tests_polyfill_tests_uninit_slice_teststest_write_fully_with_nonempty_leak_without_box_longer_lifetime {
() => {
// Module: crate::tests::polyfill_tests::uninit_slice_tests
// Provides: {"test_write_fully_with_nonempty_leak_without_box_longer_lifetime"}
// Dependencies: {}
# [test] fn test_write_fully_with_nonempty_leak_without_box_longer_lifetime () { const LEN : usize = 3 ; let mut non_empty : [u32 ; LEN] = [1 , 2 , 3] ; let longer_lifetime_non_empty : & mut [u32] = non_empty . as_mut_slice () ; { let mut uninit : [MaybeUninit < u32 > ; LEN] = [MaybeUninit :: uninit () ; LEN] ; let uninit = Uninit :: from (uninit . as_mut ()) ; assert ! (uninit . write_fully_with (| u : Uninit <'_ , u32 >| { let (ptr , len) = (u . start_ptr () , u . len ()) ; let r = unsafe { slice :: from_raw_parts_mut (longer_lifetime_non_empty . as_mut_ptr () , longer_lifetime_non_empty . len () ,) } ; assert_eq ! (r . len () , len) ; assert ! (! polyfill :: ptr :: addr_eq (r . as_ptr () , ptr)) ; Ok (r) }) . is_err ()) ; } }
};
}
