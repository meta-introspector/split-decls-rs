// Generated macro for find (function)
macro_rules! Depcrate_iter_findfind {
() => {
// Module: crate::iter::find
// Provides: {"find"}
// Dependencies: {}
pub (super) fn find < I , P > (pi : I , find_op : P) -> Option < I :: Item > where I : ParallelIterator , P : Fn (& I :: Item) -> bool + Sync , { let found = AtomicBool :: new (false) ; let consumer = FindConsumer :: new (& find_op , & found) ; pi . drive_unindexed (consumer) }
};
}
