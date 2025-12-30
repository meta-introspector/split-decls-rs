// Generated macro for KMerge (type)
macro_rules! Depcrate_kmerge_implKMerge {
() => {
// Module: crate::kmerge_impl
// Provides: {"KMerge"}
// Dependencies: {}
# [doc = " An iterator adaptor that merges an arbitrary number of base iterators in ascending order."] # [doc = " If all base iterators are sorted (ascending), the result is sorted."] # [doc = ""] # [doc = " Iterator element type is `I::Item`."] # [doc = ""] # [doc = " See [`.kmerge()`](crate::Itertools::kmerge) for more information."] pub type KMerge < I > = KMergeBy < I , KMergeByLt > ;
};
}
