// Generated macro for atomic_compare_exchange (function)
macro_rules! Depcrate_imp_atomic128_s390xatomic_compare_exchange {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"atomic_compare_exchange"}
// Dependencies: {}
# [inline] unsafe fn atomic_compare_exchange (dst : * mut u128 , old : u128 , new : u128 , _success : Ordering , _failure : Ordering ,) -> Result < u128 , u128 > { debug_assert ! (dst as usize % 16 == 0) ; let old = U128 { whole : old } ; let new = U128 { whole : new } ; let (prev_hi , prev_lo) ; let r ; let prev = unsafe { asm ! ("cdsg %r0, %r12, 0({dst})" , "ipm {r}" , dst = in (reg) ptr_reg ! (dst) , r = lateout (reg) r , inout ("r0") old . pair . hi => prev_hi , inout ("r1") old . pair . lo => prev_lo , in ("r12") new . pair . hi , in ("r13") new . pair . lo , options (nostack) ,) ; U128 { pair : Pair { hi : prev_hi , lo : prev_lo } } . whole } ; if extract_cc (r) { Ok (prev) } else { Err (prev) } }
};
}
