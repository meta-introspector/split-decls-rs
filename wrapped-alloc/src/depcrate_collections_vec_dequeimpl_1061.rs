// Generated macro for impl_1061 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_1061 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_1061"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T > FromIterator < T > for VecDeque < T > { # [track_caller] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> VecDeque < T > { SpecFromIter :: spec_from_iter (iter . into_iter ()) } }
};
}
