// Generated macro for impl_1661 (impl)
macro_rules! Depcrate_strimpl_1661 {
() => {
// Module: crate::str
// Provides: {"impl_1661"}
// Dependencies: {}
impl < 'ch > ParallelIterator for Lines < 'ch > { type Item = & 'ch str ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . 0 . par_split_terminator ('\n') . map (no_carriage_return) . drive_unindexed (consumer) } }
};
}
