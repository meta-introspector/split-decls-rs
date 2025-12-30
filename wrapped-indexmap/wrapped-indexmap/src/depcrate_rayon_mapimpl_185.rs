// Generated macro for impl_185 (impl)
macro_rules! Depcrate_rayon_mapimpl_185 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_185"}
// Dependencies: {}
impl < K : Send , V : Send > ParallelIterator for ParDrain < '_ , K , V > { type Item = (K , V) ; parallel_iterator_methods ! (Bucket :: key_value) ; }
};
}
