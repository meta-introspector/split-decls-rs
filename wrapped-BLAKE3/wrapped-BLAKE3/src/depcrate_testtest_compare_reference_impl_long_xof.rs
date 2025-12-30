// Generated macro for test_compare_reference_impl_long_xof (function)
macro_rules! Depcrate_testtest_compare_reference_impl_long_xof {
() => {
// Module: crate::test
// Provides: {"test_compare_reference_impl_long_xof"}
// Dependencies: {}
# [test] fn test_compare_reference_impl_long_xof () { let mut reference_output = [0u8 ; 32 * BLOCK_LEN - 1] ; let mut reference_hasher = reference_impl :: Hasher :: new_keyed (& TEST_KEY) ; reference_hasher . update (b"hello world") ; reference_hasher . finalize (& mut reference_output) ; let mut test_output = [0u8 ; 32 * BLOCK_LEN - 1] ; let mut test_hasher = crate :: Hasher :: new_keyed (& TEST_KEY) ; test_hasher . update (b"hello world") ; test_hasher . finalize_xof () . fill (& mut test_output) ; assert_eq ! (reference_output , test_output) ; }
};
}
