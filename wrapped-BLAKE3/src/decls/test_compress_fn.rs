macro_rules! deps {
    () => {
        CompressInPlaceFn!();
        CompressXofFn!();
    };
}

macro_rules! test_compress_fn {
    () => {
        deps!();
        pub fn test_compress_fn (compress_in_place_fn : CompressInPlaceFn , compress_xof_fn : CompressXofFn) { let initial_state = TEST_KEY_WORDS ; let block_len : u8 = 61 ; let mut block = [0 ; BLOCK_LEN] ; paint_test_input (& mut block [.. block_len as usize]) ; let counter = (5u64 << 32) + 6 ; let flags = crate :: CHUNK_END | crate :: ROOT | crate :: KEYED_HASH ; let portable_out = crate :: portable :: compress_xof (& initial_state , & block , block_len , counter as u64 , flags) ; let mut test_state = initial_state ; unsafe { compress_in_place_fn (& mut test_state , & block , block_len , counter as u64 , flags) } ; let test_state_bytes = crate :: platform :: le_bytes_from_words_32 (& test_state) ; let test_xof = unsafe { compress_xof_fn (& initial_state , & block , block_len , counter as u64 , flags) } ; assert_eq ! (& portable_out [.. 32] , & test_state_bytes [..]) ; assert_eq ! (& portable_out [..] , & test_xof [..]) ; }
    };
}

test_compress_fn!();