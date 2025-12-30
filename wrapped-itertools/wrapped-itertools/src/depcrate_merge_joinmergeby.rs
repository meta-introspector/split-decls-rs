// Generated macro for MergeBy (struct)
macro_rules! Depcrate_merge_joinMergeBy {
() => {
// Module: crate::merge_join
// Provides: {"MergeBy"}
// Dependencies: {}
# [doc = " An iterator adaptor that merges the two base iterators in ascending order."] # [doc = " If both base iterators are sorted (ascending), the result is sorted."] # [doc = ""] # [doc = " Iterator element type is `I::Item`."] # [doc = ""] # [doc = " See [`.merge_by()`](crate::Itertools::merge_by) for more information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct MergeBy < I : Iterator , J : Iterator , F > { left : PutBack < Fuse < I > > , right : PutBack < Fuse < J > > , cmp_fn : F , }
};
}
