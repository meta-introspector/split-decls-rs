// Generated macro for impl_249 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_setimpl_249 {
() => {
// Module: crate::external_trait_impls::rayon::set
// Provides: {"impl_249"}
// Dependencies: {}
impl < 'a , T : Sync , S , A : Allocator > IntoParallelIterator for & 'a HashSet < T , S , A > { type Item = & 'a T ; type Iter = ParIter < 'a , T > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_par_iter (self) -> Self :: Iter { ParIter { inner : self . map . par_keys () , } } }
};
}
