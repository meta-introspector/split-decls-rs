// Generated macro for vandq_u8_hide (function)
macro_rules! Depcrate_neonvandq_u8_hide {
() => {
// Module: crate::neon
// Provides: {"vandq_u8_hide"}
// Dependencies: {}
# [doc = " Equivalent to `vandq_u8`, but hidden from the compiler."] # [doc = ""] # [doc = " The use of inline assembly instead of an intrinsic prevents a sufficiently"] # [doc = " smart compiler from short circuiting the computation once the mask becomes"] # [doc = " all zeros."] # [must_use] # [inline (always)] fn vandq_u8_hide (a : uint8x16_t , b : uint8x16_t) -> uint8x16_t { let mut c ; # [cfg (target_arch = "aarch64")] unsafe { asm ! ("and {c:v}.16b, {a:v}.16b, {b:v}.16b" , c = lateout (vreg) c , a = in (vreg) a , b = in (vreg) b , options (pure , nomem , preserves_flags , nostack)) ; } c }
};
}
