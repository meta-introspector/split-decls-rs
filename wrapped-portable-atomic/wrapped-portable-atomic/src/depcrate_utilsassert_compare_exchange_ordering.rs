// Generated macro for assert_compare_exchange_ordering (function)
macro_rules! Depcrate_utilsassert_compare_exchange_ordering {
() => {
// Module: crate::utils
// Provides: {"assert_compare_exchange_ordering"}
// Dependencies: {}
# [inline] # [cfg_attr (all (debug_assertions , not (portable_atomic_no_track_caller)) , track_caller)] pub (crate) fn assert_compare_exchange_ordering (success : Ordering , failure : Ordering) { match success { Ordering :: AcqRel | Ordering :: Acquire | Ordering :: Relaxed | Ordering :: Release | Ordering :: SeqCst => { } _ => unreachable ! () , } match failure { Ordering :: Acquire | Ordering :: Relaxed | Ordering :: SeqCst => { } Ordering :: Release => panic ! ("there is no such thing as a release failure ordering") , Ordering :: AcqRel => panic ! ("there is no such thing as an acquire-release failure ordering") , _ => unreachable ! () , } }
};
}
