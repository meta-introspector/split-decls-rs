// Generated macro for impl_1389 (impl)
macro_rules! Depcrate_range_inclusiveimpl_1389 {
() => {
// Module: crate::range_inclusive
// Provides: {"impl_1389"}
// Dependencies: {}
impl < T : RangeInteger > ParallelIterator for Iter < T > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < T > , { T :: drive_unindexed (self , consumer) } # [inline] fn opt_len (& self) -> Option < usize > { T :: opt_len (self) } }
};
}
