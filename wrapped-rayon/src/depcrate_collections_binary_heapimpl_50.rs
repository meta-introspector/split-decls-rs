// Generated macro for impl_50 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_50 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_50"}
// Dependencies: {}
impl < T > Drop for Drain < '_ , T > { fn drop (& mut self) { if ! self . heap . is_empty () { self . heap . drain () ; } } }
};
}
