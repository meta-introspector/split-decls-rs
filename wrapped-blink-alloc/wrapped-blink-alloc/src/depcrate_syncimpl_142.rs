// Generated macro for impl_142 (impl)
macro_rules! Depcrate_syncimpl_142 {
() => {
// Module: crate::sync
// Provides: {"impl_142"}
// Dependencies: {}
impl < A > Drop for LocalBlinkAlloc < '_ , A > where A : Allocator , { fn drop (& mut self) { self . shared . update_max_local_alloc (self . arena . last_chunk_size ()) ; self . arena . reset_leak (false) ; } }
};
}
