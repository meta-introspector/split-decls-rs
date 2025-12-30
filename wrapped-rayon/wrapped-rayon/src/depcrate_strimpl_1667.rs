// Generated macro for impl_1667 (impl)
macro_rules! Depcrate_strimpl_1667 {
() => {
// Module: crate::str
// Provides: {"impl_1667"}
// Dependencies: {}
impl < 'ch > ParallelIterator for SplitAsciiWhitespace < 'ch > { type Item = & 'ch str ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . 0 . par_split (is_ascii_whitespace) . filter (not_empty) . drive_unindexed (consumer) } }
};
}
