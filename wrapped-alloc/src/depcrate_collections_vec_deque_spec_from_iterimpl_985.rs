// Generated macro for impl_985 (impl)
macro_rules! Depcrate_collections_vec_deque_spec_from_iterimpl_985 {
() => {
// Module: crate::collections::vec_deque::spec_from_iter
// Provides: {"impl_985"}
// Dependencies: {}
impl < T , I > SpecFromIter < T , I > for VecDeque < T > where I : Iterator < Item = T > , { # [track_caller] default fn spec_from_iter (iterator : I) -> Self { crate :: vec :: Vec :: from_iter (iterator) . into () } }
};
}
