// Generated macro for debug_assert_vmovdqa_atomic (macro)
macro_rules! Depcrate_imp_atomic128_x86_64debug_assert_vmovdqa_atomic {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"debug_assert_vmovdqa_atomic"}
// Dependencies: {}
# [cfg (not (any (portable_atomic_no_outline_atomics , target_env = "sgx")))] # [cfg (target_feature = "sse")] macro_rules ! debug_assert_vmovdqa_atomic { () => { { debug_assert_cmpxchg16b ! () ; debug_assert ! (detect :: detect () . vmovdqa_atomic ()) ; } } ; }
};
}
