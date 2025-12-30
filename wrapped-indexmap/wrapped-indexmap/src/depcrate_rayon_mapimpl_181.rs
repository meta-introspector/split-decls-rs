// Generated macro for impl_181 (impl)
macro_rules! Depcrate_rayon_mapimpl_181 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_181"}
// Dependencies: {}
impl < 'a , K : Sync + Send , V : Send > ParallelIterator for ParIterMut < 'a , K , V > { type Item = (& 'a K , & 'a mut V) ; parallel_iterator_methods ! (Bucket :: ref_mut) ; }
};
}
