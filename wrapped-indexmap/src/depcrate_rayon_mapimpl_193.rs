// Generated macro for impl_193 (impl)
macro_rules! Depcrate_rayon_mapimpl_193 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_193"}
// Dependencies: {}
impl < 'a , K : Sync , V : Sync > ParallelIterator for ParKeys < 'a , K , V > { type Item = & 'a K ; parallel_iterator_methods ! (Bucket :: key_ref) ; }
};
}
