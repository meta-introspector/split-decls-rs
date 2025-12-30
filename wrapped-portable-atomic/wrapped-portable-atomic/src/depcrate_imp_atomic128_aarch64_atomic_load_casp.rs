// Generated macro for _atomic_load_casp (function)
macro_rules! Depcrate_imp_atomic128_aarch64_atomic_load_casp {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"_atomic_load_casp"}
// Dependencies: {}
# [cfg (any (test , not (any (target_feature = "lse2" , portable_atomic_target_feature = "lse2"))))] # [cfg (any (target_feature = "lse" , portable_atomic_target_feature = "lse"))] # [inline] unsafe fn _atomic_load_casp (src : * mut u128 , order : Ordering) -> u128 { debug_assert ! (src as usize % 16 == 0) ; debug_assert_lse ! () ; unsafe { let (out_lo , out_hi) ; macro_rules ! atomic_load { ($ acquire : tt , $ release : tt) => { asm ! (start_lse ! () , concat ! ("casp" , $ acquire , $ release , " x2, x3, x2, x3, [{src}]") , src = in (reg) ptr_reg ! (src) , inout ("x2") 0_u64 => out_lo , inout ("x3") 0_u64 => out_hi , options (nostack , preserves_flags) ,) } ; } match order { Ordering :: Relaxed => atomic_load ! ("" , "") , Ordering :: Acquire => atomic_load ! ("a" , "") , Ordering :: SeqCst => atomic_load ! ("a" , "l") , _ => unreachable ! () , } U128 { pair : Pair { lo : out_lo , hi : out_hi } } . whole } }
};
}
