// Generated macro for fence (function)
macro_rules! Depcrate_imp_msp430fence {
() => {
// Module: crate::imp::msp430
// Provides: {"fence"}
// Dependencies: {}
# [doc = " An atomic fence."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `order` is [`Relaxed`](Ordering::Relaxed)."] # [inline] # [cfg_attr (all (debug_assertions , not (portable_atomic_no_track_caller)) , track_caller)] pub fn fence (order : Ordering) { match order { Ordering :: Relaxed => panic ! ("there is no such thing as a relaxed fence") , _ => compiler_fence (order) , } }
};
}
