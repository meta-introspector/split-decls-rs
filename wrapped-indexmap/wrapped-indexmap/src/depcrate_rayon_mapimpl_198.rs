// Generated macro for impl_198 (impl)
macro_rules! Depcrate_rayon_mapimpl_198 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_198"}
// Dependencies: {}
impl < 'a , K : Sync , V : Sync > ParallelIterator for ParValues < 'a , K , V > { type Item = & 'a V ; parallel_iterator_methods ! (Bucket :: value_ref) ; }
};
}
