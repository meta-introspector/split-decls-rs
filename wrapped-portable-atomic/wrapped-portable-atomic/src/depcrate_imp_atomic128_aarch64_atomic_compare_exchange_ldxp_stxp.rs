// Generated macro for _atomic_compare_exchange_ldxp_stxp (function)
macro_rules! Depcrate_imp_atomic128_aarch64_atomic_compare_exchange_ldxp_stxp {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"_atomic_compare_exchange_ldxp_stxp"}
// Dependencies: {}
# [cfg (any (test , not (any (target_feature = "lse" , portable_atomic_target_feature = "lse"))))] # [inline] unsafe fn _atomic_compare_exchange_ldxp_stxp (dst : * mut u128 , old : u128 , new : u128 , success : Ordering , failure : Ordering ,) -> u128 { debug_assert ! (dst as usize % 16 == 0) ; let order = crate :: utils :: upgrade_success_ordering (success , failure) ; unsafe { let old = U128 { whole : old } ; let new = U128 { whole : new } ; let (mut prev_lo , mut prev_hi) ; macro_rules ! cmpxchg { ($ acquire : tt , $ release : tt , $ fence : tt) => { asm ! ("2:" , concat ! ("ld" , $ acquire , "xp {prev_lo}, {prev_hi}, [{dst}]") , "cmp {prev_lo}, {old_lo}" , "cset {r:w}, ne" , "cmp {prev_hi}, {old_hi}" , "cinc {r:w}, {r:w}, ne" , "cbz {r:w}, 3f" , concat ! ("st" , $ release , "xp {r:w}, {prev_lo}, {prev_hi}, [{dst}]") , "cbnz {r:w}, 2b" , "b 4f" , "3:" , concat ! ("st" , $ release , "xp {r:w}, {new_lo}, {new_hi}, [{dst}]") , "cbnz {r:w}, 2b" , "4:" , $ fence , dst = in (reg) ptr_reg ! (dst) , old_lo = in (reg) old . pair . lo , old_hi = in (reg) old . pair . hi , new_lo = in (reg) new . pair . lo , new_hi = in (reg) new . pair . hi , prev_lo = out (reg) prev_lo , prev_hi = out (reg) prev_hi , r = out (reg) _ , options (nostack) ,) } ; } atomic_rmw ! (cmpxchg , order , write = success) ; U128 { pair : Pair { lo : prev_lo , hi : prev_hi } } . whole } }
};
}
