// Generated macro for impl_986 (impl)
macro_rules! Depcrate_collections_vec_deque_spec_from_iterimpl_986 {
() => {
// Module: crate::collections::vec_deque::spec_from_iter
// Provides: {"impl_986"}
// Dependencies: {}
# [cfg (not (test))] impl < T > SpecFromIter < T , crate :: vec :: IntoIter < T > > for VecDeque < T > { # [inline] fn spec_from_iter (iterator : crate :: vec :: IntoIter < T >) -> Self { iterator . into_vecdeque () } }
};
}
