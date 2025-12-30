// Generated macro for atomic_swap_pwr8 (function)
macro_rules! Depcrate_imp_atomic128_powerpc64atomic_swap_pwr8 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"atomic_swap_pwr8"}
// Dependencies: {}
# [inline] unsafe fn atomic_swap_pwr8 (dst : * mut u128 , val : u128 , order : Ordering) -> u128 { debug_assert ! (dst as usize % 16 == 0) ; debug_assert_pwr8 ! () ; let val = U128 { whole : val } ; let (mut prev_hi , mut prev_lo) ; unsafe { macro_rules ! swap { ($ acquire : tt , $ release : tt) => { asm ! (start_pwr8 ! () , $ release , "2:" , "lqarx %r6, 0, {dst}" , "stqcx. %r8, 0, {dst}" , "bne %cr0, 2b" , $ acquire , end_pwr8 ! () , dst = in (reg_nonzero) ptr_reg ! (dst) , out ("r6") prev_hi , out ("r7") prev_lo , in ("r8") val . pair . hi , in ("r9") val . pair . lo , out ("cr0") _ , options (nostack , preserves_flags) ,) } ; } atomic_rmw ! (swap , order) ; U128 { pair : Pair { hi : prev_hi , lo : prev_lo } } . whole } }
};
}
