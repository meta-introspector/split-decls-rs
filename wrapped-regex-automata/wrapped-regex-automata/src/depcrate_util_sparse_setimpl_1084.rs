// Generated macro for impl_1084 (impl)
macro_rules! Depcrate_util_sparse_setimpl_1084 {
() => {
// Module: crate::util::sparse_set
// Provides: {"impl_1084"}
// Dependencies: {}
impl < 'a > Iterator for SparseSetIter < 'a > { type Item = StateID ; # [cfg_attr (feature = "perf-inline" , inline (always))] fn next (& mut self) -> Option < StateID > { self . 0 . next () . copied () } }
};
}
