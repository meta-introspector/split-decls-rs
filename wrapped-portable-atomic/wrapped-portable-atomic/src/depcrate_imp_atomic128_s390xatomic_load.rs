// Generated macro for atomic_load (function)
macro_rules! Depcrate_imp_atomic128_s390xatomic_load {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"atomic_load"}
// Dependencies: {}
# [inline] unsafe fn atomic_load (src : * mut u128 , _order : Ordering) -> u128 { debug_assert ! (src as usize % 16 == 0) ; let (out_hi , out_lo) ; unsafe { asm ! ("lpq %r0, 0({src})" , src = in (reg) ptr_reg ! (src) , out ("r0") out_hi , out ("r1") out_lo , options (nostack , preserves_flags) ,) ; U128 { pair : Pair { hi : out_hi , lo : out_lo } } . whole } }
};
}
