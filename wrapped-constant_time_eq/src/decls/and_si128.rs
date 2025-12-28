macro_rules! and_si128 {
    () => {
        # [doc = " Equivalent to `_mm_and_si128`, but hidden from the compiler."] # [doc = ""] # [doc = " The use of inline assembly instead of an intrinsic prevents a sufficiently"] # [doc = " smart compiler from short circuiting the computation once the mask becomes"] # [doc = " all zeros."] # [must_use] # [inline (always)] fn and_si128 (a : __m128i , b : __m128i) -> __m128i { let mut c ; if cfg ! (target_feature = "avx") { unsafe { asm ! ("vpand {c}, {a}, {b}" , c = lateout (xmm_reg) c , a = in (xmm_reg) a , b = in (xmm_reg) b , options (pure , nomem , preserves_flags , nostack)) ; } } else { unsafe { asm ! ("pand {a}, {b}" , a = inlateout (xmm_reg) a => c , b = in (xmm_reg) b , options (pure , nomem , preserves_flags , nostack)) ; } } c }
    };
}

and_si128!()