// Generated macro for test_write_fully_with_nonempty_box_leak (function)
macro_rules! Depcrate_tests_polyfill_tests_uninit_slice_teststest_write_fully_with_nonempty_box_leak {
() => {
// Module: crate::tests::polyfill_tests::uninit_slice_tests
// Provides: {"test_write_fully_with_nonempty_box_leak"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [test] fn test_write_fully_with_nonempty_box_leak () { use alloc :: boxed :: Box ; let mut uninit : [MaybeUninit < u8 > ; 1] = [MaybeUninit :: uninit () ; 1] ; let uninit = Uninit :: from (uninit . as_mut ()) ; const ARBITRARY : u8 = 1 ; assert ! (uninit . write_fully_with (| u | { let (ptr , len) = (u . start_ptr () , u . len ()) ; let r : & mut [_] = Box :: leak (Box :: new ([ARBITRARY])) ; assert_eq ! (r . len () , len) ; assert ! (! polyfill :: ptr :: addr_eq (r . as_ptr () , ptr)) ; Ok (r) }) . is_err ()) ; }
};
}
