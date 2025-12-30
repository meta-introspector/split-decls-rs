// Generated macro for find_first (function)
macro_rules! Depcrate_iter_find_first_lastfind_first {
() => {
// Module: crate::iter::find_first_last
// Provides: {"find_first"}
// Dependencies: {}
pub (super) fn find_first < I , P > (pi : I , find_op : P) -> Option < I :: Item > where I : ParallelIterator , P : Fn (& I :: Item) -> bool + Sync , { let best_found = AtomicUsize :: new (usize :: MAX) ; let consumer = FindConsumer :: new (& find_op , MatchPosition :: Leftmost , & best_found) ; pi . drive_unindexed (consumer) }
};
}
