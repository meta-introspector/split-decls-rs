// Generated macro for _atomic_swap_casp (function)
macro_rules! Depcrate_imp_atomic128_aarch64_atomic_swap_casp {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"_atomic_swap_casp"}
// Dependencies: {}
# [cfg (any (test , not (portable_atomic_ll_sc_rmw)))] # [cfg (any (target_feature = "lse" , portable_atomic_target_feature = "lse"))] # [inline] unsafe fn _atomic_swap_casp (dst : * mut u128 , val : u128 , order : Ordering) -> u128 { debug_assert ! (dst as usize % 16 == 0) ; debug_assert_lse ! () ; unsafe { let val = U128 { whole : val } ; let (mut prev_lo , mut prev_hi) ; macro_rules ! swap { ($ acquire : tt , $ release : tt , $ fence : tt) => { asm ! (start_lse ! () , "ldp x4, x5, [{dst}]" , "2:" , "mov {tmp_lo}, x4" , "mov {tmp_hi}, x5" , concat ! ("casp" , $ acquire , $ release , " x4, x5, x2, x3, [{dst}]") , "cmp {tmp_hi}, x5" , "ccmp {tmp_lo}, x4, #0, eq" , "b.ne 2b" , $ fence , dst = in (reg) ptr_reg ! (dst) , tmp_lo = out (reg) _ , tmp_hi = out (reg) _ , out ("x4") prev_lo , out ("x5") prev_hi , in ("x2") val . pair . lo , in ("x3") val . pair . hi , options (nostack) ,) } ; } atomic_rmw ! (swap , order) ; U128 { pair : Pair { lo : prev_lo , hi : prev_hi } } . whole } }
};
}
