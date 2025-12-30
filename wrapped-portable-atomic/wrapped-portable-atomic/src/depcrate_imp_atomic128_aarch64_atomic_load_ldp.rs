// Generated macro for _atomic_load_ldp (function)
macro_rules! Depcrate_imp_atomic128_aarch64_atomic_load_ldp {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"_atomic_load_ldp"}
// Dependencies: {}
# [cfg (any (target_feature = "lse2" , portable_atomic_target_feature = "lse2" , not (portable_atomic_no_outline_atomics) ,))] # [inline] unsafe fn _atomic_load_ldp (src : * mut u128 , order : Ordering) -> u128 { debug_assert ! (src as usize % 16 == 0) ; debug_assert_lse2 ! () ; unsafe { let (out_lo , out_hi) ; macro_rules ! atomic_load_relaxed { ($ acquire : tt) => { { asm ! ("ldp {out_lo}, {out_hi}, [{src}]" , $ acquire , src = in (reg) ptr_reg ! (src) , out_hi = lateout (reg) out_hi , out_lo = lateout (reg) out_lo , options (nostack , preserves_flags) ,) ; U128 { pair : Pair { lo : out_lo , hi : out_hi } } . whole } } ; } match order { # [cfg (any (target_feature = "rcpc3" , portable_atomic_target_feature = "rcpc3"))] Ordering :: Acquire | Ordering :: SeqCst => _atomic_load_ldiapp (src , order) , Ordering :: Relaxed => atomic_load_relaxed ! ("") , # [cfg (not (any (target_feature = "rcpc3" , portable_atomic_target_feature = "rcpc3")))] Ordering :: Acquire => atomic_load_relaxed ! ("dmb ishld") , # [cfg (not (any (target_feature = "rcpc3" , portable_atomic_target_feature = "rcpc3")))] Ordering :: SeqCst => { asm ! ("ldar {tmp}, [{src}]" , "ldp {out_lo}, {out_hi}, [{src}]" , "dmb ishld" , src = in (reg) ptr_reg ! (src) , out_hi = lateout (reg) out_hi , out_lo = lateout (reg) out_lo , tmp = out (reg) _ , options (nostack , preserves_flags) ,) ; U128 { pair : Pair { lo : out_lo , hi : out_hi } } . whole } _ => unreachable ! () , } } }
};
}
