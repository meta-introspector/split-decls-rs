// Generated macro for atomic_load_pwr8 (function)
macro_rules! Depcrate_imp_atomic128_powerpc64atomic_load_pwr8 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"atomic_load_pwr8"}
// Dependencies: {}
# [inline] unsafe fn atomic_load_pwr8 (src : * mut u128 , order : Ordering) -> u128 { debug_assert ! (src as usize % 16 == 0) ; debug_assert_pwr8 ! () ; let (out_hi , out_lo) ; unsafe { macro_rules ! atomic_load_acquire { ($ release : tt) => { asm ! (start_pwr8 ! () , $ release , "lq %r4, 0({src})" , "cmpw %r4, %r4" , "bne- %cr0, 2f" , "2:" , "isync" , end_pwr8 ! () , src = in (reg_nonzero) ptr_reg ! (src) , out ("r4") out_hi , out ("r5") out_lo , out ("cr0") _ , options (nostack , preserves_flags) ,) } ; } match order { Ordering :: Relaxed => { asm ! (start_pwr8 ! () , "lq %r4, 0({src})" , end_pwr8 ! () , src = in (reg_nonzero) ptr_reg ! (src) , out ("r4") out_hi , out ("r5") out_lo , options (nostack , preserves_flags) ,) ; } Ordering :: Acquire => atomic_load_acquire ! ("") , Ordering :: SeqCst => atomic_load_acquire ! ("sync") , _ => unreachable ! () , } U128 { pair : Pair { hi : out_hi , lo : out_lo } } . whole } }
};
}
