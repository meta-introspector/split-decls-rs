// Generated macro for test_hash_many_avx512 (function)
macro_rules! Depcrate_testtest_hash_many_avx512 {
() => {
// Module: crate::test
// Provides: {"test_hash_many_avx512"}
// Dependencies: {}
# [test] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] fn test_hash_many_avx512 () { if ! crate :: avx512_detected () { return ; } test_hash_many_fn (crate :: ffi :: x86 :: blake3_hash_many_avx512) ; }
};
}
