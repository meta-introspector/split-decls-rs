// Generated macro for atomic_and (function)
macro_rules! Depcrate_imp_atomic128_aarch64atomic_and {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"atomic_and"}
// Dependencies: {}
# [cfg (any (target_feature = "lse128" , portable_atomic_target_feature = "lse128"))] # [inline] unsafe fn atomic_and (dst : * mut u128 , val : u128 , order : Ordering) -> u128 { debug_assert ! (dst as usize % 16 == 0) ; unsafe { let val = U128 { whole : ! val } ; let (prev_lo , prev_hi) ; # [cfg (not (portable_atomic_pre_llvm_16))] macro_rules ! clear { ($ acquire : tt , $ release : tt , $ fence : tt) => { asm ! (start_lse128 ! () , concat ! ("ldclrp" , $ acquire , $ release , " {val_lo}, {val_hi}, [{dst}]") , $ fence , dst = in (reg) ptr_reg ! (dst) , val_lo = inout (reg) val . pair . lo => prev_lo , val_hi = inout (reg) val . pair . hi => prev_hi , options (nostack , preserves_flags) ,) } ; } # [cfg (not (portable_atomic_pre_llvm_16))] atomic_rmw ! (clear , order) ; # [cfg (portable_atomic_pre_llvm_16)] macro_rules ! clear { ($ order : tt , $ fence : tt) => { asm ! (concat ! (".inst 0x19" , $ order , "11008") , $ fence , in ("x0") ptr_reg ! (dst) , inout ("x8") val . pair . lo => prev_lo , inout ("x1") val . pair . hi => prev_hi , options (nostack , preserves_flags) ,) } ; } # [cfg (portable_atomic_pre_llvm_16)] atomic_rmw_inst ! (clear , order) ; U128 { pair : Pair { lo : prev_lo , hi : prev_hi } } . whole } }
};
}
