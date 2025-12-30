// Generated macro for impl_1046 (impl)
macro_rules! Depcrate_iter_skip_anyimpl_1046 {
() => {
// Module: crate::iter::skip_any
// Provides: {"impl_1046"}
// Dependencies: {}
impl < I > ParallelIterator for SkipAny < I > where I : ParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = SkipAnyConsumer { base : consumer , count : & AtomicUsize :: new (self . count) , } ; self . base . drive_unindexed (consumer1) } }
};
}
