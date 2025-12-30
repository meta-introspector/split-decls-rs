// Generated macro for impl_1793 (impl)
macro_rules! Depcrate_vec_into_iterimpl_1793 {
() => {
// Module: crate::vec::into_iter
// Provides: {"impl_1793"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] unsafe impl < T > AsVecIntoIter for IntoIter < T > { type Item = T ; fn as_into_iter (& mut self) -> & mut IntoIter < Self :: Item > { self } }
};
}
