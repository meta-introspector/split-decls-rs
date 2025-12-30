// Generated macro for find_last (function)
macro_rules! Depcrate_iter_find_first_lastfind_last {
() => {
// Module: crate::iter::find_first_last
// Provides: {"find_last"}
// Dependencies: {}
pub (super) fn find_last < I , P > (pi : I , find_op : P) -> Option < I :: Item > where I : ParallelIterator , P : Fn (& I :: Item) -> bool + Sync , { let best_found = AtomicUsize :: new (0) ; let consumer = FindConsumer :: new (& find_op , MatchPosition :: Rightmost , & best_found) ; pi . drive_unindexed (consumer) }
};
}
