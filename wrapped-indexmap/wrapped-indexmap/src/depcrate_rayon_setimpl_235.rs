// Generated macro for impl_235 (impl)
macro_rules! Depcrate_rayon_setimpl_235 {
() => {
// Module: crate::rayon::set
// Provides: {"impl_235"}
// Dependencies: {}
impl < 'a , T : Sync > ParallelIterator for ParIter < 'a , T > { type Item = & 'a T ; parallel_iterator_methods ! (Bucket :: key_ref) ; }
};
}
