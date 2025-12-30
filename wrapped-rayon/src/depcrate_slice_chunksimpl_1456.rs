// Generated macro for impl_1456 (impl)
macro_rules! Depcrate_slice_chunksimpl_1456 {
() => {
// Module: crate::slice::chunks
// Provides: {"impl_1456"}
// Dependencies: {}
impl < 'data , T : Sync > ParallelIterator for Chunks < 'data , T > { type Item = & 'data [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
