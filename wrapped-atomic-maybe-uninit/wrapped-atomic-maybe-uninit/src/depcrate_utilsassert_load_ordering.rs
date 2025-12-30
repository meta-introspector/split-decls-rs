// Generated macro for assert_load_ordering (function)
macro_rules! Depcrate_utilsassert_load_ordering {
() => {
// Module: crate::utils
// Provides: {"assert_load_ordering"}
// Dependencies: {}
# [inline] # [cfg_attr (debug_assertions , track_caller)] pub (crate) fn assert_load_ordering (order : Ordering) { match order { Ordering :: Acquire | Ordering :: Relaxed | Ordering :: SeqCst => { } Ordering :: Release => panic ! ("there is no such thing as a release load") , Ordering :: AcqRel => panic ! ("there is no such thing as an acquire-release load") , _ => unreachable ! () , } }
};
}
