// Generated macro for impl_1116 (impl)
macro_rules! Depcrate_iter_take_anyimpl_1116 {
() => {
// Module: crate::iter::take_any
// Provides: {"impl_1116"}
// Dependencies: {}
impl < I > ParallelIterator for TakeAny < I > where I : ParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = TakeAnyConsumer { base : consumer , count : & AtomicUsize :: new (self . count) , } ; self . base . drive_unindexed (consumer1) } }
};
}
