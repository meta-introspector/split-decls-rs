// Generated macro for test_compress_fn (function)
macro_rules! Depcrate_testtest_compress_fn {
() => {
// Module: crate::test
// Provides: {"test_compress_fn"}
// Dependencies: {}
pub fn test_compress_fn (compress_in_place_fn : CompressInPlaceFn , compress_xof_fn : CompressXofFn) { let initial_state = TEST_KEY_WORDS ; let block_len : u8 = 61 ; let mut block = [0 ; BLOCK_LEN] ; paint_test_input (& mut block [.. block_len as usize]) ; let counter = (5u64 << 32) + 6 ; let flags = CHUNK_END | ROOT | KEYED_HASH ; let mut portable_out = [0 ; 64] ; unsafe { crate :: ffi :: blake3_compress_xof_portable (initial_state . as_ptr () , block . as_ptr () , block_len , counter , flags , portable_out . as_mut_ptr () ,) ; } let mut test_state = initial_state ; unsafe { compress_in_place_fn (test_state . as_mut_ptr () , block . as_ptr () , block_len , counter , flags ,) } ; let test_state_bytes = le_bytes_from_words_32 (& test_state) ; let mut test_xof = [0 ; 64] ; unsafe { compress_xof_fn (initial_state . as_ptr () , block . as_ptr () , block_len , counter , flags , test_xof . as_mut_ptr () ,) } ; assert_eq ! (& portable_out [.. 32] , & test_state_bytes [..]) ; assert_eq ! (& portable_out [..] , & test_xof [..]) ; }
};
}
