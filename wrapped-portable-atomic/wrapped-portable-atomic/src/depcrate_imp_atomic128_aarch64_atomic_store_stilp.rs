// Generated macro for _atomic_store_stilp (function)
macro_rules! Depcrate_imp_atomic128_aarch64_atomic_store_stilp {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"_atomic_store_stilp"}
// Dependencies: {}
# [cfg (any (target_feature = "lse2" , portable_atomic_target_feature = "lse2" , not (portable_atomic_no_outline_atomics) ,))] # [cfg (any (target_feature = "rcpc3" , portable_atomic_target_feature = "rcpc3" , all (not (portable_atomic_no_outline_atomics) , not (any (target_feature = "lse2" , portable_atomic_target_feature = "lse2")) ,) ,))] # [inline] unsafe fn _atomic_store_stilp (dst : * mut u128 , val : u128 , order : Ordering) { debug_assert ! (dst as usize % 16 == 0) ; debug_assert_lse2 ! () ; debug_assert_rcpc3 ! () ; unsafe { macro_rules ! atomic_store { ($ acquire : tt) => { { let val = U128 { whole : val } ; # [cfg (not (portable_atomic_pre_llvm_16))] asm ! (start_rcpc3 ! () , "stilp {val_lo}, {val_hi}, [{dst}]" , $ acquire , dst = in (reg) ptr_reg ! (dst) , val_lo = in (reg) val . pair . lo , val_hi = in (reg) val . pair . hi , options (nostack , preserves_flags) ,) ; # [cfg (portable_atomic_pre_llvm_16)] asm ! (".inst 0xd9031802" , $ acquire , in ("x0") ptr_reg ! (dst) , in ("x2") val . pair . lo , in ("x3") val . pair . hi , options (nostack , preserves_flags) ,) ; } } ; } match order { Ordering :: Release => atomic_store ! ("") , Ordering :: SeqCst => atomic_store ! ("dmb ish") , _ => unreachable ! () , } } }
};
}
