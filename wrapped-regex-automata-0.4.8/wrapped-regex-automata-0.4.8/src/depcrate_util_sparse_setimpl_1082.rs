// Generated macro for impl_1082 (impl)
macro_rules! Depcrate_util_sparse_setimpl_1082 {
() => {
// Module: crate::util::sparse_set
// Provides: {"impl_1082"}
// Dependencies: {}
impl < 'a > Iterator for SparseSetIter < 'a > { type Item = StateID ; # [cfg_attr (feature = "perf-inline" , inline (always))] fn next (& mut self) -> Option < StateID > { self . 0 . next () . map (| & id | id) } }
};
}
