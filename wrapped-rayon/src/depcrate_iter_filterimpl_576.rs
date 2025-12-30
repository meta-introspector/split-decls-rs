// Generated macro for impl_576 (impl)
macro_rules! Depcrate_iter_filterimpl_576 {
() => {
// Module: crate::iter::filter
// Provides: {"impl_576"}
// Dependencies: {}
impl < I , P > ParallelIterator for Filter < I , P > where I : ParallelIterator , P : Fn (& I :: Item) -> bool + Sync + Send , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = FilterConsumer :: new (consumer , & self . filter_op) ; self . base . drive_unindexed (consumer1) } }
};
}
