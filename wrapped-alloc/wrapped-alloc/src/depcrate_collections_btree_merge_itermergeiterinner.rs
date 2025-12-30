// Generated macro for MergeIterInner (struct)
macro_rules! Depcrate_collections_btree_merge_iterMergeIterInner {
() => {
// Module: crate::collections::btree::merge_iter
// Provides: {"MergeIterInner"}
// Dependencies: {}
# [doc = " Core of an iterator that merges the output of two strictly ascending iterators,"] # [doc = " for instance a union or a symmetric difference."] pub (super) struct MergeIterInner < I : Iterator > { a : I , b : I , peeked : Option < Peeked < I > > , }
};
}
