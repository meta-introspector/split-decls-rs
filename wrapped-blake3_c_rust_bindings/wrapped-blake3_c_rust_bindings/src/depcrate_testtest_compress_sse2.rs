// Generated macro for test_compress_sse2 (function)
macro_rules! Depcrate_testtest_compress_sse2 {
() => {
// Module: crate::test
// Provides: {"test_compress_sse2"}
// Dependencies: {}
# [test] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] fn test_compress_sse2 () { if ! crate :: sse2_detected () { return ; } test_compress_fn (crate :: ffi :: x86 :: blake3_compress_in_place_sse2 , crate :: ffi :: x86 :: blake3_compress_xof_sse2 ,) ; }
};
}
