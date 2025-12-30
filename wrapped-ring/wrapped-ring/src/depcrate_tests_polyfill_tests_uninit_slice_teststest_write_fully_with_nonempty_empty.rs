// Generated macro for test_write_fully_with_nonempty_empty (function)
macro_rules! Depcrate_tests_polyfill_tests_uninit_slice_teststest_write_fully_with_nonempty_empty {
() => {
// Module: crate::tests::polyfill_tests::uninit_slice_tests
// Provides: {"test_write_fully_with_nonempty_empty"}
// Dependencies: {}
# [test] fn test_write_fully_with_nonempty_empty () { let mut uninit : [MaybeUninit < u8 > ; 1] = [MaybeUninit :: uninit () ; 1] ; let uninit = Uninit :: from (uninit . as_mut ()) ; assert ! (uninit . write_fully_with (| uninit | Ok (uninit . write_copy_of_slice (& []) . unwrap_or_else (| _ | unreachable ! ()) . ignore_uninit () . into_written ())) . is_err ()) ; }
};
}
