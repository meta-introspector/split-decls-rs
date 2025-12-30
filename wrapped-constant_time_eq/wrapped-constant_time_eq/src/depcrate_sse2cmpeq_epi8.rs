// Generated macro for cmpeq_epi8 (function)
macro_rules! Depcrate_sse2cmpeq_epi8 {
() => {
// Module: crate::sse2
// Provides: {"cmpeq_epi8"}
// Dependencies: {}
# [doc = " Equivalent to `_mm_cmpeq_epi8`, but hidden from the compiler."] # [doc = ""] # [doc = " The use of inline assembly instead of an intrinsic prevents a sufficiently"] # [doc = " smart compiler from computing the mask in other ways which might not be"] # [doc = " constant time (for instance, looping through the input and using branching"] # [doc = " to set the vector elements)."] # [must_use] # [inline (always)] fn cmpeq_epi8 (a : __m128i , b : __m128i) -> __m128i { let mut c ; if cfg ! (target_feature = "avx") { unsafe { asm ! ("vpcmpeqb {c}, {a}, {b}" , c = lateout (xmm_reg) c , a = in (xmm_reg) a , b = in (xmm_reg) b , options (pure , nomem , preserves_flags , nostack)) ; } } else { unsafe { asm ! ("pcmpeqb {a}, {b}" , a = inlateout (xmm_reg) a => c , b = in (xmm_reg) b , options (pure , nomem , preserves_flags , nostack)) ; } } c }
};
}
