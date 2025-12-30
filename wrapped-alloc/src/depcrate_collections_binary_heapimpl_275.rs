// Generated macro for impl_275 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_275 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_275"}
// Dependencies: {}
impl < T : Ord , A : Allocator > Drop for RebuildOnDrop < '_ , T , A > { fn drop (& mut self) { self . heap . rebuild_tail (self . rebuild_from) ; } }
};
}
