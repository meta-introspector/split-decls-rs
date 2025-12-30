// Generated macro for atomic_store_pwr8 (function)
macro_rules! Depcrate_imp_atomic128_powerpc64atomic_store_pwr8 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"atomic_store_pwr8"}
// Dependencies: {}
# [inline] unsafe fn atomic_store_pwr8 (dst : * mut u128 , val : u128 , order : Ordering) { debug_assert ! (dst as usize % 16 == 0) ; debug_assert_pwr8 ! () ; let val = U128 { whole : val } ; unsafe { macro_rules ! atomic_store { ($ release : tt) => { asm ! (start_pwr8 ! () , $ release , "stq %r4, 0({dst})" , end_pwr8 ! () , dst = in (reg_nonzero) ptr_reg ! (dst) , in ("r4") val . pair . hi , in ("r5") val . pair . lo , options (nostack , preserves_flags) ,) } ; } match order { Ordering :: Relaxed => atomic_store ! ("") , Ordering :: Release => atomic_store ! ("lwsync") , Ordering :: SeqCst => atomic_store ! ("sync") , _ => unreachable ! () , } } }
};
}
