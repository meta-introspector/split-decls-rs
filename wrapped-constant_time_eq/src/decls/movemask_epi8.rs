macro_rules! movemask_epi8 {
    () => {
        # [doc = " Equivalent to `_mm_movemask_epi8`, but hidden from the compiler."] # [doc = ""] # [doc = " The use of inline assembly instead of an intrinsic prevents a sufficiently"] # [doc = " smart compiler from extracting the mask in other ways which might not be"] # [doc = " constant time (for instance, looping through the elements of the vector)."] # [must_use] # [inline (always)] fn movemask_epi8 (a : __m128i) -> u32 { let mut mask ; if cfg ! (target_feature = "avx") { unsafe { asm ! ("vpmovmskb {mask:e}, {a}" , mask = lateout (reg) mask , a = in (xmm_reg) a , options (pure , nomem , preserves_flags , nostack)) ; } } else { unsafe { asm ! ("pmovmskb {mask:e}, {a}" , mask = lateout (reg) mask , a = in (xmm_reg) a , options (pure , nomem , preserves_flags , nostack)) ; } } mask }
    };
}

movemask_epi8!()