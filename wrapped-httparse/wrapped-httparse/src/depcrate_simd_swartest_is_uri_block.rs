// Generated macro for test_is_uri_block (function)
macro_rules! Depcrate_simd_swartest_is_uri_block {
() => {
// Module: crate::simd::swar
// Provides: {"test_is_uri_block"}
// Dependencies: {}
# [test] fn test_is_uri_block () { let is_uri_block = | b | match_uri_char_8_swar (b) == BLOCK_SIZE ; for b in 0 .. 33_u8 { assert ! (! is_uri_block ([b ; BLOCK_SIZE]) , "b={}" , b) ; } for b in 33 ..= 126_u8 { assert ! (is_uri_block ([b ; BLOCK_SIZE]) , "b={}" , b) ; } assert ! (! is_uri_block ([b'\x7F' ; BLOCK_SIZE]) , "b={}" , b'\x7F') ; for b in 128 ..= 255_u8 { assert ! (is_uri_block ([b ; BLOCK_SIZE]) , "b={}" , b) ; } }
};
}
