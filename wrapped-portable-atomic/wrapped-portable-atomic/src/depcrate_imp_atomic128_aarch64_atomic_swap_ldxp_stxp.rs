// Generated macro for _atomic_swap_ldxp_stxp (function)
macro_rules! Depcrate_imp_atomic128_aarch64_atomic_swap_ldxp_stxp {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"_atomic_swap_ldxp_stxp"}
// Dependencies: {}
# [cfg (any (test , not (all (any (target_feature = "lse" , portable_atomic_target_feature = "lse") , not (portable_atomic_ll_sc_rmw) ,))))] # [inline] unsafe fn _atomic_swap_ldxp_stxp (dst : * mut u128 , val : u128 , order : Ordering) -> u128 { debug_assert ! (dst as usize % 16 == 0) ; unsafe { let val = U128 { whole : val } ; let (mut prev_lo , mut prev_hi) ; macro_rules ! swap { ($ acquire : tt , $ release : tt , $ fence : tt) => { asm ! ("2:" , concat ! ("ld" , $ acquire , "xp {prev_lo}, {prev_hi}, [{dst}]") , concat ! ("st" , $ release , "xp {r:w}, {val_lo}, {val_hi}, [{dst}]") , "cbnz {r:w}, 2b" , $ fence , dst = in (reg) ptr_reg ! (dst) , val_lo = in (reg) val . pair . lo , val_hi = in (reg) val . pair . hi , prev_lo = out (reg) prev_lo , prev_hi = out (reg) prev_hi , r = out (reg) _ , options (nostack , preserves_flags) ,) } ; } atomic_rmw ! (swap , order) ; U128 { pair : Pair { lo : prev_lo , hi : prev_hi } } . whole } }
};
}
