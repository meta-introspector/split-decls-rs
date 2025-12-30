// Generated macro for impl_634 (impl)
macro_rules! Depcrate_indexerimpl_634 {
() => {
// Module: crate::indexer
// Provides: {"impl_634"}
// Dependencies: {}
impl Drop for Indexer < '_ > { fn drop (& mut self) { unsafe { raw :: git_indexer_free (self . raw) ; drop (Box :: from_raw (self . progress_payload_ptr)) } } }
};
}
