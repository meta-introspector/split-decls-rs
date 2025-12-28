macro_rules! vceqq_u8_hide {
    () => {
        # [doc = " Equivalent to `vceqq_u8`, but hidden from the compiler."] # [doc = ""] # [doc = " The use of inline assembly instead of an intrinsic prevents a sufficiently"] # [doc = " smart compiler from computing the mask in other ways which might not be"] # [doc = " constant time (for instance, looping through the input and using branching"] # [doc = " to set the vector elements)."] # [must_use] # [inline (always)] fn vceqq_u8_hide (a : uint8x16_t , b : uint8x16_t) -> uint8x16_t { let mut c ; # [cfg (target_arch = "aarch64")] unsafe { asm ! ("cmeq {c:v}.16b, {a:v}.16b, {b:v}.16b" , c = lateout (vreg) c , a = in (vreg) a , b = in (vreg) b , options (pure , nomem , preserves_flags , nostack)) ; } c }
    };
}

vceqq_u8_hide!();