// Generated macro for test_xof_many_fn (function)
macro_rules! Depcrate_testtest_xof_many_fn {
() => {
// Module: crate::test
// Provides: {"test_xof_many_fn"}
// Dependencies: {}
# [allow (unused)] pub fn test_xof_many_fn (xof_many_function : XofManyFunction) { let mut block = [0 ; BLOCK_LEN] ; let block_len = 42 ; crate :: test :: paint_test_input (& mut block [.. block_len]) ; let cv = [40 , 41 , 42 , 43 , 44 , 45 , 46 , 47] ; let flags = crate :: KEYED_HASH ; let initial_counters = [0 , u32 :: MAX as u64 , i32 :: MAX as u64] ; for counter in initial_counters { # [cfg (feature = "std")] dbg ! (counter) ; const OUTPUT_SIZE : usize = 31 * BLOCK_LEN ; let mut portable_out = [0u8 ; OUTPUT_SIZE] ; for (i , out_block) in portable_out . chunks_exact_mut (64) . enumerate () { out_block . copy_from_slice (& crate :: portable :: compress_xof (& cv , & block , block_len as u8 , counter + i as u64 , flags ,)) ; } let mut test_out = [0u8 ; OUTPUT_SIZE] ; unsafe { xof_many_function (& cv , & block , block_len as u8 , counter , flags , & mut test_out) ; } assert_eq ! (portable_out , test_out) ; } for block_count in 1 ..= 32 { let mut array = [0 ; BLOCK_LEN * 33] ; let output_start = 17 ; let output_len = block_count * BLOCK_LEN ; let output_end = output_start + output_len ; let output = & mut array [output_start .. output_end] ; unsafe { xof_many_function (& cv , & block , block_len as u8 , 0 , flags , output) ; } for i in 0 .. array . len () { if i < output_start || output_end <= i { assert_eq ! (0 , array [i] , "index {i}") ; } } } }
};
}
