// Generated macro for atomic_load_zacas (function)
macro_rules! Depcrate_imp_atomic64_riscv32atomic_load_zacas {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"atomic_load_zacas"}
// Dependencies: {}
# [inline] unsafe fn atomic_load_zacas (src : * mut u64 , order : Ordering) -> u64 { debug_assert ! (src as usize % 8 == 0) ; debug_assert_zacas ! () ; let (out_lo , out_hi) ; unsafe { # [cfg (not (portable_atomic_pre_llvm_20))] macro_rules ! load { ($ fence : tt , $ asm_order : tt) => { asm ! (start_zacas ! () , $ fence , concat ! ("amocas.d" , $ asm_order , " a2, a2, 0({src})") , end_zacas ! () , src = in (reg) ptr_reg ! (src) , inout ("a2") 0_u32 => out_lo , inout ("a3") 0_u32 => out_hi , options (nostack , preserves_flags) ,) } ; } # [cfg (not (portable_atomic_pre_llvm_20))] atomic_rmw_amocas_order ! (load , order) ; # [cfg (portable_atomic_pre_llvm_20)] macro_rules ! load { ($ fence : tt , $ insn_order : tt) => { asm ! ($ fence , concat ! (".4byte 0x2" , $ insn_order , "c5362f") , in ("a0") ptr_reg ! (src) , inout ("a2") 0_u32 => out_lo , inout ("a3") 0_u32 => out_hi , options (nostack , preserves_flags) ,) } ; } # [cfg (portable_atomic_pre_llvm_20)] atomic_rmw_amocas_order_insn ! (load , order) ; U64 { pair : Pair { lo : out_lo , hi : out_hi } } . whole } }
};
}
