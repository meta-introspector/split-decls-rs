// Generated macro for partition (function)
macro_rules! Depcrate_iter_unzippartition {
() => {
// Module: crate::iter::unzip
// Provides: {"partition"}
// Dependencies: {}
# [doc = " Partitions the items of a parallel iterator into a pair of arbitrary"] # [doc = " `ParallelExtend` containers."] # [doc = ""] # [doc = " This is called by `ParallelIterator::partition`."] pub (super) fn partition < I , A , B , P > (pi : I , predicate : P) -> (A , B) where I : ParallelIterator , A : Default + Send + ParallelExtend < I :: Item > , B : Default + Send + ParallelExtend < I :: Item > , P : Fn (& I :: Item) -> bool + Sync + Send , { execute (pi , Partition { predicate }) }
};
}
