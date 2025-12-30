// Generated macro for impl_838 (impl)
macro_rules! Depcrate_iter_lenimpl_838 {
() => {
// Module: crate::iter::len
// Provides: {"impl_838"}
// Dependencies: {}
impl < I > ParallelIterator for MinLen < I > where I : IndexedParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
