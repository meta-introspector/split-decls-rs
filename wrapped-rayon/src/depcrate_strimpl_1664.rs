// Generated macro for impl_1664 (impl)
macro_rules! Depcrate_strimpl_1664 {
() => {
// Module: crate::str
// Provides: {"impl_1664"}
// Dependencies: {}
impl < 'ch > ParallelIterator for SplitWhitespace < 'ch > { type Item = & 'ch str ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . 0 . par_split (char :: is_whitespace) . filter (not_empty) . drive_unindexed (consumer) } }
};
}
