// Generated macro for byte_wise_atomic_load (function)
macro_rules! Depcrate_imp_atomic128_riscv64byte_wise_atomic_load {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"byte_wise_atomic_load"}
// Dependencies: {}
# [inline] unsafe fn byte_wise_atomic_load (src : * const u128) -> u128 { let (out_lo , out_hi) ; unsafe { asm ! ("ld {out_lo}, ({src})" , "ld {out_hi}, 8({src})" , src = in (reg) ptr_reg ! (src) , out_lo = out (reg) out_lo , out_hi = out (reg) out_hi , options (pure , nostack , preserves_flags , readonly) ,) ; U128 { pair : Pair { lo : out_lo , hi : out_hi } } . whole } }
};
}
