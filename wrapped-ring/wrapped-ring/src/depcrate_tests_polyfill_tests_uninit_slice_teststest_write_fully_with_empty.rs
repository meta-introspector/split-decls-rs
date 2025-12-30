// Generated macro for test_write_fully_with_empty (function)
macro_rules! Depcrate_tests_polyfill_tests_uninit_slice_teststest_write_fully_with_empty {
() => {
// Module: crate::tests::polyfill_tests::uninit_slice_tests
// Provides: {"test_write_fully_with_empty"}
// Dependencies: {}
# [test] fn test_write_fully_with_empty () { let mut uninit : [MaybeUninit < u8 > ; 0] = [] ; let uninit = Uninit :: from (uninit . as_mut ()) ; let empty : & mut [u8] = & mut [] ; assert_eq ! (Some (empty) , uninit . write_fully_with (| u | { let (_ptr , len) = (u . start_ptr () , u . len ()) ; let r = & mut [] ; assert_eq ! (len , 0) ; assert_eq ! (r . len () , len) ; Ok (r) }) . ok ()) ; }
};
}
