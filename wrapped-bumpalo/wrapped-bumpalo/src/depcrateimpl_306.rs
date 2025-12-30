// Generated macro for impl_306 (impl)
macro_rules! Depcrateimpl_306 {
() => {
// Module: crate
// Provides: {"impl_306"}
// Dependencies: {}
impl < const MIN_ALIGN : usize > Drop for Bump < MIN_ALIGN > { fn drop (& mut self) { unsafe { dealloc_chunk_list (self . current_chunk_footer . get ()) ; } } }
};
}
