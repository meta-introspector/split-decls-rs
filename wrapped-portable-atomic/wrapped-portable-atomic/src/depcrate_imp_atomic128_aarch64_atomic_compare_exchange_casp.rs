// Generated macro for _atomic_compare_exchange_casp (function)
macro_rules! Depcrate_imp_atomic128_aarch64_atomic_compare_exchange_casp {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"_atomic_compare_exchange_casp"}
// Dependencies: {}
# [cfg (any (target_feature = "lse" , portable_atomic_target_feature = "lse" , not (portable_atomic_no_outline_atomics) ,))] # [inline] unsafe fn _atomic_compare_exchange_casp (dst : * mut u128 , old : u128 , new : u128 , success : Ordering , failure : Ordering ,) -> u128 { debug_assert ! (dst as usize % 16 == 0) ; debug_assert_lse ! () ; let order = crate :: utils :: upgrade_success_ordering (success , failure) ; unsafe { let old = U128 { whole : old } ; let new = U128 { whole : new } ; let (prev_lo , prev_hi) ; macro_rules ! cmpxchg { ($ acquire : tt , $ release : tt , $ fence : tt) => { asm ! (start_lse ! () , concat ! ("casp" , $ acquire , $ release , " x6, x7, x4, x5, [{dst}]") , $ fence , dst = in (reg) ptr_reg ! (dst) , inout ("x6") old . pair . lo => prev_lo , inout ("x7") old . pair . hi => prev_hi , in ("x4") new . pair . lo , in ("x5") new . pair . hi , options (nostack , preserves_flags) ,) } ; } atomic_rmw ! (cmpxchg , order , write = success) ; U128 { pair : Pair { lo : prev_lo , hi : prev_hi } } . whole } }
};
}
