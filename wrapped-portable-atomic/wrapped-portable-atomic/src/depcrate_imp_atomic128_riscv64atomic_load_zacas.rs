// Generated macro for atomic_load_zacas (function)
macro_rules! Depcrate_imp_atomic128_riscv64atomic_load_zacas {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"atomic_load_zacas"}
// Dependencies: {}
# [inline] unsafe fn atomic_load_zacas (src : * mut u128 , order : Ordering) -> u128 { debug_assert ! (src as usize % 16 == 0) ; debug_assert_zacas ! () ; let (out_lo , out_hi) ; unsafe { # [cfg (not (portable_atomic_pre_llvm_20))] macro_rules ! load { ($ fence : tt , $ asm_order : tt) => { asm ! (start_zacas ! () , $ fence , concat ! ("amocas.q" , $ asm_order , " a2, a2, 0({src})") , end_zacas ! () , src = in (reg) ptr_reg ! (src) , inout ("a2") 0_u64 => out_lo , inout ("a3") 0_u64 => out_hi , options (nostack , preserves_flags) ,) } ; } # [cfg (not (portable_atomic_pre_llvm_20))] atomic_rmw_amocas_order ! (load , order) ; # [cfg (portable_atomic_pre_llvm_20)] macro_rules ! load { ($ fence : tt , $ insn_order : tt) => { asm ! ($ fence , concat ! (".4byte 0x2" , $ insn_order , "c5462f") , in ("a0") ptr_reg ! (src) , inout ("a2") 0_u64 => out_lo , inout ("a3") 0_u64 => out_hi , options (nostack , preserves_flags) ,) } ; } # [cfg (portable_atomic_pre_llvm_20)] atomic_rmw_amocas_order_insn ! (load , order) ; U128 { pair : Pair { lo : out_lo , hi : out_hi } } . whole } }
};
}
