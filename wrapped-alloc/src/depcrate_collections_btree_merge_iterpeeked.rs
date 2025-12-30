// Generated macro for Peeked (enum)
macro_rules! Depcrate_collections_btree_merge_iterPeeked {
() => {
// Module: crate::collections::btree::merge_iter
// Provides: {"Peeked"}
// Dependencies: {}
# [doc = " Benchmarks faster than wrapping both iterators in a Peekable,"] # [doc = " probably because we can afford to impose a FusedIterator bound."] # [derive (Clone , Debug)] enum Peeked < I : Iterator > { A (I :: Item) , B (I :: Item) , }
};
}
