// Generated macro for impl_590 (impl)
macro_rules! Depcrate_iter_filter_mapimpl_590 {
() => {
// Module: crate::iter::filter_map
// Provides: {"impl_590"}
// Dependencies: {}
impl < I , P , R > ParallelIterator for FilterMap < I , P > where I : ParallelIterator , P : Fn (I :: Item) -> Option < R > + Sync + Send , R : Send , { type Item = R ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer = FilterMapConsumer :: new (consumer , & self . filter_op) ; self . base . drive_unindexed (consumer) } }
};
}
