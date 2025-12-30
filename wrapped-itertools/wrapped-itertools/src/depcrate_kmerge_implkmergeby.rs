// Generated macro for KMergeBy (struct)
macro_rules! Depcrate_kmerge_implKMergeBy {
() => {
// Module: crate::kmerge_impl
// Provides: {"KMergeBy"}
// Dependencies: {}
# [doc = " An iterator adaptor that merges an arbitrary number of base iterators"] # [doc = " according to an ordering function."] # [doc = ""] # [doc = " Iterator element type is `I::Item`."] # [doc = ""] # [doc = " See [`.kmerge_by()`](crate::Itertools::kmerge_by) for more"] # [doc = " information."] # [must_use = "this iterator adaptor is not lazy but does nearly nothing unless consumed"] pub struct KMergeBy < I , F > where I : Iterator , { heap : Vec < HeadTail < I > > , less_than : F , }
};
}
