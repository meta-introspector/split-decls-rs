// Generated macro for atomic_store (function)
macro_rules! Depcrate_imp_atomic128_s390xatomic_store {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"atomic_store"}
// Dependencies: {}
# [inline] unsafe fn atomic_store (dst : * mut u128 , val : u128 , order : Ordering) { debug_assert ! (dst as usize % 16 == 0) ; let val = U128 { whole : val } ; unsafe { macro_rules ! atomic_store { ($ acquire : expr) => { asm ! ("stpq %r0, 0({dst})" , $ acquire , dst = in (reg) ptr_reg ! (dst) , in ("r0") val . pair . hi , in ("r1") val . pair . lo , options (nostack , preserves_flags) ,) } ; } match order { Ordering :: Relaxed | Ordering :: Release => atomic_store ! ("") , Ordering :: SeqCst => atomic_store ! (serialization ! ()) , _ => unreachable ! () , } } }
};
}
