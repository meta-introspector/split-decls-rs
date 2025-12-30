// Generated macro for test_write_fully_with_nonempty_short (function)
macro_rules! Depcrate_tests_polyfill_tests_uninit_slice_teststest_write_fully_with_nonempty_short {
() => {
// Module: crate::tests::polyfill_tests::uninit_slice_tests
// Provides: {"test_write_fully_with_nonempty_short"}
// Dependencies: {}
# [test] fn test_write_fully_with_nonempty_short () { const LEN : usize = 3 ; let mut uninit : [MaybeUninit < u8 > ; 3] = [MaybeUninit :: uninit () ; LEN] ; let uninit = Uninit :: from (uninit . as_mut ()) ; const ARBITRARY : [u8 ; LEN] = [1 , 1 , 3] ; assert ! (uninit . write_fully_with (| u | { let (ptr , len) = (u . start_ptr () , u . len ()) ; let (_ , r) = u . write_copy_of_slice (& ARBITRARY) . unwrap_or_else (| LenMismatchError { .. } | unreachable ! ()) . uninit_empty () . unwrap_or_else (| _ | unreachable ! ()) . into_written () . split_last_mut () . unwrap () ; assert_ne ! (r . len () , len) ; assert ! (polyfill :: ptr :: addr_eq (r . as_ptr () , ptr)) ; Ok (r) }) . is_err ()) ; }
};
}
