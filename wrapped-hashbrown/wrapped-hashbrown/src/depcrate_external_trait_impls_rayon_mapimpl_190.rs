// Generated macro for impl_190 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_190 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_190"}
// Dependencies: {}
impl < K : Send , V : Send , S , A : Allocator + Send > IntoParallelIterator for HashMap < K , V , S , A > { type Item = (K , V) ; type Iter = IntoParIter < K , V , A > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_par_iter (self) -> Self :: Iter { IntoParIter { inner : self . table . into_par_iter () , } } }
};
}
