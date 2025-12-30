// Generated macro for impl_987 (impl)
macro_rules! Depcrate_collections_vec_deque_spec_from_iterimpl_987 {
() => {
// Module: crate::collections::vec_deque::spec_from_iter
// Provides: {"impl_987"}
// Dependencies: {}
impl < T > SpecFromIter < T , IntoIter < T > > for VecDeque < T > { # [inline] fn spec_from_iter (iterator : IntoIter < T >) -> Self { iterator . into_vecdeque () } }
};
}
