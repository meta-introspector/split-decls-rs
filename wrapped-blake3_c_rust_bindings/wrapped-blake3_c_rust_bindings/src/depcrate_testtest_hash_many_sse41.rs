// Generated macro for test_hash_many_sse41 (function)
macro_rules! Depcrate_testtest_hash_many_sse41 {
() => {
// Module: crate::test
// Provides: {"test_hash_many_sse41"}
// Dependencies: {}
# [test] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] fn test_hash_many_sse41 () { if ! crate :: sse41_detected () { return ; } test_hash_many_fn (crate :: ffi :: x86 :: blake3_hash_many_sse41) ; }
};
}
