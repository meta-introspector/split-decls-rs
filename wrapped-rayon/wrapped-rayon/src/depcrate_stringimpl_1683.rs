// Generated macro for impl_1683 (impl)
macro_rules! Depcrate_stringimpl_1683 {
() => {
// Module: crate::string
// Provides: {"impl_1683"}
// Dependencies: {}
impl < 'a > ParallelIterator for Drain < 'a > { type Item = char ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . string [self . range . clone ()] . par_chars () . drive_unindexed (consumer) } }
};
}
