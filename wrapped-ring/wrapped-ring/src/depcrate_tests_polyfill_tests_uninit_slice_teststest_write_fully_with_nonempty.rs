// Generated macro for test_write_fully_with_nonempty (function)
macro_rules! Depcrate_tests_polyfill_tests_uninit_slice_teststest_write_fully_with_nonempty {
() => {
// Module: crate::tests::polyfill_tests::uninit_slice_tests
// Provides: {"test_write_fully_with_nonempty"}
// Dependencies: {}
# [test] fn test_write_fully_with_nonempty () { let mut uninit : [MaybeUninit < u8 > ; 1] = [MaybeUninit :: uninit () ; 1] ; let uninit = Uninit :: from (uninit . as_mut ()) ; const ARBITRARY : u8 = 1 ; assert_eq ! (Some ([ARBITRARY] . as_mut_slice ()) , uninit . write_fully_with (| uninit | Ok (uninit . write_copy_of_slice (& [ARBITRARY]) . unwrap_or_else (| _ | unreachable ! ()) . uninit_empty () . unwrap_or_else (| _ | unreachable ! ()) . into_written ())) . ok ()) ; }
};
}
