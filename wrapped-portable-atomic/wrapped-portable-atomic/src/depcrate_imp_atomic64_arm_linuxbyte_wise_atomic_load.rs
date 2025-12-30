// Generated macro for byte_wise_atomic_load (function)
macro_rules! Depcrate_imp_atomic64_arm_linuxbyte_wise_atomic_load {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"byte_wise_atomic_load"}
// Dependencies: {}
# [inline] unsafe fn byte_wise_atomic_load (src : * const u64) -> u64 { unsafe { let (out_lo , out_hi) ; asm ! ("ldr {out_lo}, [{src}]" , "ldr {out_hi}, [{src}, #4]" , src = in (reg) src , out_lo = out (reg) out_lo , out_hi = out (reg) out_hi , options (pure , nostack , preserves_flags , readonly) ,) ; U64 { pair : Pair { lo : out_lo , hi : out_hi } } . whole } }
};
}
