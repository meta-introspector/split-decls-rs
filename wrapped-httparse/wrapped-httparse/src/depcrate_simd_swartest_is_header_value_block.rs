// Generated macro for test_is_header_value_block (function)
macro_rules! Depcrate_simd_swartest_is_header_value_block {
() => {
// Module: crate::simd::swar
// Provides: {"test_is_header_value_block"}
// Dependencies: {}
# [test] fn test_is_header_value_block () { let is_header_value_block = | b | match_header_value_char_8_swar (b) == BLOCK_SIZE ; for b in 0 .. 32_u8 { assert ! (! is_header_value_block ([b ; BLOCK_SIZE]) , "b={}" , b) ; } for b in 32 ..= 126_u8 { assert ! (is_header_value_block ([b ; BLOCK_SIZE]) , "b={}" , b) ; } assert ! (! is_header_value_block ([b'\x7F' ; BLOCK_SIZE]) , "b={}" , b'\x7F') ; for b in 128 ..= 255_u8 { assert ! (is_header_value_block ([b ; BLOCK_SIZE]) , "b={}" , b) ; } # [cfg (target_pointer_width = "64")] { assert ! (! is_header_value_block (* b"foo.com\n")) ; assert ! (! is_header_value_block (* b"o.com\r\nU")) ; } }
};
}
