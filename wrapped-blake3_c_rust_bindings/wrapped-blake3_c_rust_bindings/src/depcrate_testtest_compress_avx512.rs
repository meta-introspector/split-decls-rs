// Generated macro for test_compress_avx512 (function)
macro_rules! Depcrate_testtest_compress_avx512 {
() => {
// Module: crate::test
// Provides: {"test_compress_avx512"}
// Dependencies: {}
# [test] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] fn test_compress_avx512 () { if ! crate :: avx512_detected () { return ; } test_compress_fn (crate :: ffi :: x86 :: blake3_compress_in_place_avx512 , crate :: ffi :: x86 :: blake3_compress_xof_avx512 ,) ; }
};
}
