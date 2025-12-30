// Generated macro for impl_248 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_setimpl_248 {
() => {
// Module: crate::external_trait_impls::rayon::set
// Provides: {"impl_248"}
// Dependencies: {}
impl < T : Send , S , A : Allocator + Send > IntoParallelIterator for HashSet < T , S , A > { type Item = T ; type Iter = IntoParIter < T , A > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_par_iter (self) -> Self :: Iter { IntoParIter { inner : self . map . into_par_iter () , } } }
};
}
