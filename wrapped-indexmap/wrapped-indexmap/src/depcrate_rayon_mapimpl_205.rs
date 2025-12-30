// Generated macro for impl_205 (impl)
macro_rules! Depcrate_rayon_mapimpl_205 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_205"}
// Dependencies: {}
impl < 'a , K : Send , V : Send > ParallelIterator for ParValuesMut < 'a , K , V > { type Item = & 'a mut V ; parallel_iterator_methods ! (Bucket :: value_mut) ; }
};
}
