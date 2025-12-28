macro_rules! vshrn_n_u16_4_hide {
    () => {
        # [doc = " Equivalent to `vshrn_n_u16(..., 4)`, but hidden from the compiler."] # [doc = ""] # [doc = " The use of inline assembly instead of an intrinsic prevents a sufficiently"] # [doc = " smart compiler from extracting the mask in other ways which might not be"] # [doc = " constant time (for instance, looping through the elements of the vector)."] # [must_use] # [inline (always)] fn vshrn_n_u16_4_hide (a : uint16x8_t) -> uint8x8_t { let mut mask ; # [cfg (target_arch = "aarch64")] unsafe { asm ! ("shrn {mask:v}.8b, {a:v}.8h, #{n}" , mask = lateout (vreg) mask , a = in (vreg) a , n = const 4 , options (pure , nomem , preserves_flags , nostack)) ; } mask }
    };
}

vshrn_n_u16_4_hide!()