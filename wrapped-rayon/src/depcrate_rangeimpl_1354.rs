// Generated macro for impl_1354 (impl)
macro_rules! Depcrate_rangeimpl_1354 {
() => {
// Module: crate::range
// Provides: {"impl_1354"}
// Dependencies: {}
impl < T : RangeInteger > ParallelIterator for Iter < T > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < T > , { T :: drive_unindexed (self , consumer) } # [inline] fn opt_len (& self) -> Option < usize > { T :: opt_len (self) } }
};
}
