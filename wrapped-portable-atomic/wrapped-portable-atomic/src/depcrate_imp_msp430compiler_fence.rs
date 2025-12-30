// Generated macro for compiler_fence (function)
macro_rules! Depcrate_imp_msp430compiler_fence {
() => {
// Module: crate::imp::msp430
// Provides: {"compiler_fence"}
// Dependencies: {}
# [doc = " A compiler memory fence."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `order` is [`Relaxed`](Ordering::Relaxed)."] # [inline] # [cfg_attr (all (debug_assertions , not (portable_atomic_no_track_caller)) , track_caller)] pub fn compiler_fence (order : Ordering) { match order { Ordering :: Relaxed => panic ! ("there is no such thing as a relaxed compiler fence") , _ => { } } unsafe { # [cfg (not (portable_atomic_no_asm))] asm ! ("" , options (nostack , preserves_flags)) ; # [cfg (portable_atomic_no_asm)] llvm_asm ! ("" ::: "memory" : "volatile") ; } }
};
}
