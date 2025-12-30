// Generated macro for Merge (type)
macro_rules! Depcrate_merge_joinMerge {
() => {
// Module: crate::merge_join
// Provides: {"Merge"}
// Dependencies: {}
# [doc = " An iterator adaptor that merges the two base iterators in ascending order."] # [doc = " If both base iterators are sorted (ascending), the result is sorted."] # [doc = ""] # [doc = " Iterator element type is `I::Item`."] # [doc = ""] # [doc = " See [`.merge()`](crate::Itertools::merge_by) for more information."] pub type Merge < I , J > = MergeBy < I , J , MergeLte > ;
};
}
