// Generated macro for assert_store_ordering (function)
macro_rules! Depcrate_utilsassert_store_ordering {
() => {
// Module: crate::utils
// Provides: {"assert_store_ordering"}
// Dependencies: {}
# [inline] # [cfg_attr (all (debug_assertions , not (portable_atomic_no_track_caller)) , track_caller)] pub (crate) fn assert_store_ordering (order : Ordering) { match order { Ordering :: Release | Ordering :: Relaxed | Ordering :: SeqCst => { } Ordering :: Acquire => panic ! ("there is no such thing as an acquire store") , Ordering :: AcqRel => panic ! ("there is no such thing as an acquire-release store") , _ => unreachable ! () , } }
};
}
