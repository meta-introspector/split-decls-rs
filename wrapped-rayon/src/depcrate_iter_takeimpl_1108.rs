// Generated macro for impl_1108 (impl)
macro_rules! Depcrate_iter_takeimpl_1108 {
() => {
// Module: crate::iter::take
// Provides: {"impl_1108"}
// Dependencies: {}
impl < I > ParallelIterator for Take < I > where I : IndexedParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
