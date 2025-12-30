// Generated macro for impl_175 (impl)
macro_rules! Depcrate_rayon_mapimpl_175 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_175"}
// Dependencies: {}
impl < 'a , K : Sync , V : Sync > ParallelIterator for ParIter < 'a , K , V > { type Item = (& 'a K , & 'a V) ; parallel_iterator_methods ! (Bucket :: refs) ; }
};
}
