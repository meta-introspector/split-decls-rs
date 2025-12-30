// Generated macro for impl_168 (impl)
macro_rules! Depcrate_rayon_mapimpl_168 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_168"}
// Dependencies: {}
impl < K : Send , V : Send > ParallelIterator for IntoParIter < K , V > { type Item = (K , V) ; parallel_iterator_methods ! (Bucket :: key_value) ; }
};
}
