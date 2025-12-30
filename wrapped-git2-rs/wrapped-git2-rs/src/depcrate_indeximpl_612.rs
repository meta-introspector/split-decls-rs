// Generated macro for impl_612 (impl)
macro_rules! Depcrate_indeximpl_612 {
() => {
// Module: crate::index
// Provides: {"impl_612"}
// Dependencies: {}
impl < 'index > Drop for IndexConflicts < 'index > { fn drop (& mut self) { unsafe { raw :: git_index_conflict_iterator_free (self . conflict_iter) } } }
};
}
