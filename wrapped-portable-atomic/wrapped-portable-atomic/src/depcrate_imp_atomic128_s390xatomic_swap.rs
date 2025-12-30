// Generated macro for atomic_swap (function)
macro_rules! Depcrate_imp_atomic128_s390xatomic_swap {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"atomic_swap"}
// Dependencies: {}
# [inline] unsafe fn atomic_swap (dst : * mut u128 , val : u128 , _order : Ordering) -> u128 { debug_assert ! (dst as usize % 16 == 0) ; let val = U128 { whole : val } ; let (mut prev_hi , mut prev_lo) ; unsafe { asm ! ("lg %r0, 8({dst})" , "lg %r1, 0({dst})" , "2:" , "cdsg %r0, %r12, 0({dst})" , "jl 2b" , dst = in (reg) ptr_reg ! (dst) , out ("r0") prev_hi , out ("r1") prev_lo , in ("r12") val . pair . hi , in ("r13") val . pair . lo , options (nostack) ,) ; U128 { pair : Pair { hi : prev_hi , lo : prev_lo } } . whole } }
};
}
