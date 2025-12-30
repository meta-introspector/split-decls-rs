// Generated macro for atomic_load_vmovdqa (function)
macro_rules! Depcrate_imp_atomic128_x86_64atomic_load_vmovdqa {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"atomic_load_vmovdqa"}
// Dependencies: {}
# [cfg (not (any (portable_atomic_no_outline_atomics , target_env = "sgx")))] # [cfg (target_feature = "sse")] # [target_feature (enable = "avx")] # [inline] unsafe fn atomic_load_vmovdqa (src : * mut u128) -> u128 { debug_assert ! (src as usize % 16 == 0) ; debug_assert_vmovdqa_atomic ! () ; unsafe { let out : core :: arch :: x86_64 :: __m128i ; asm ! (concat ! ("vmovdqa {out}, xmmword ptr [{src" , ptr_modifier ! () , "}]") , src = in (reg) src , out = out (xmm_reg) out , options (nostack , preserves_flags) ,) ; core :: mem :: transmute (out) } }
};
}
