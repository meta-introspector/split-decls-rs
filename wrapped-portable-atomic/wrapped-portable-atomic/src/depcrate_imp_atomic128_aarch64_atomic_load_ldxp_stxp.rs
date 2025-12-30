// Generated macro for _atomic_load_ldxp_stxp (function)
macro_rules! Depcrate_imp_atomic128_aarch64_atomic_load_ldxp_stxp {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"_atomic_load_ldxp_stxp"}
// Dependencies: {}
# [cfg (any (test , all (not (any (target_feature = "lse2" , portable_atomic_target_feature = "lse2")) , not (any (target_feature = "lse" , portable_atomic_target_feature = "lse")) ,) ,))] # [inline] unsafe fn _atomic_load_ldxp_stxp (src : * mut u128 , order : Ordering) -> u128 { debug_assert ! (src as usize % 16 == 0) ; unsafe { let (mut out_lo , mut out_hi) ; macro_rules ! atomic_load { ($ acquire : tt , $ release : tt) => { asm ! ("2:" , concat ! ("ld" , $ acquire , "xp {out_lo}, {out_hi}, [{src}]") , concat ! ("st" , $ release , "xp {r:w}, {out_lo}, {out_hi}, [{src}]") , "cbnz {r:w}, 2b" , src = in (reg) ptr_reg ! (src) , out_lo = out (reg) out_lo , out_hi = out (reg) out_hi , r = out (reg) _ , options (nostack , preserves_flags) ,) } ; } match order { Ordering :: Relaxed => atomic_load ! ("" , "") , Ordering :: Acquire => atomic_load ! ("a" , "") , Ordering :: SeqCst => atomic_load ! ("a" , "l") , _ => unreachable ! () , } U128 { pair : Pair { lo : out_lo , hi : out_hi } } . whole } }
};
}
