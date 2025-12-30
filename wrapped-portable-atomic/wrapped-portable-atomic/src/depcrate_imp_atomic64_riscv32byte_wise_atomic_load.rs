// Generated macro for byte_wise_atomic_load (function)
macro_rules! Depcrate_imp_atomic64_riscv32byte_wise_atomic_load {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"byte_wise_atomic_load"}
// Dependencies: {}
# [inline] unsafe fn byte_wise_atomic_load (src : * const u64) -> u64 { let (out_lo , out_hi) ; unsafe { asm ! ("lw {out_lo}, ({src})" , "lw {out_hi}, 4({src})" , src = in (reg) ptr_reg ! (src) , out_lo = out (reg) out_lo , out_hi = out (reg) out_hi , options (pure , nostack , preserves_flags , readonly) ,) ; U64 { pair : Pair { lo : out_lo , hi : out_hi } } . whole } }
};
}
