// Generated macro for test_write_fully_with_nonempty_leak_without_box_same_lifeime (function)
macro_rules! Depcrate_tests_polyfill_tests_uninit_slice_teststest_write_fully_with_nonempty_leak_without_box_same_lifeime {
() => {
// Module: crate::tests::polyfill_tests::uninit_slice_tests
// Provides: {"test_write_fully_with_nonempty_leak_without_box_same_lifeime"}
// Dependencies: {}
# [test] fn test_write_fully_with_nonempty_leak_without_box_same_lifeime () { const LEN : usize = 3 ; let mut uninit : [MaybeUninit < u32 > ; LEN * 2] = [MaybeUninit :: uninit () ; LEN * 2] ; let (uninit , after) = uninit . split_at_mut (LEN) ; let after = Uninit :: from (after) . write_copy_of_slice (& [1 , 2 , 3]) . unwrap_or_else (| _ | unreachable ! ()) . uninit_empty () . unwrap_or_else (| _ | unreachable ! ()) . into_written () ; assert_eq ! (uninit . len () , after . len ()) ; let uninit = Uninit :: from (uninit) ; assert ! (uninit . write_fully_with (| u : Uninit <'_ , u32 >| { let (ptr , len) = (u . start_ptr () , u . len ()) ; let r = unsafe { slice :: from_raw_parts_mut (after . as_mut_ptr () , after . len ()) } ; assert_eq ! (r . len () , len) ; assert ! (! polyfill :: ptr :: addr_eq (r . as_ptr () , ptr)) ; Ok (r) }) . is_err ()) ; }
};
}
