// Generated macro for impl_165 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_165 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_165"}
// Dependencies: {}
impl < K , V > Clone for ParIter < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Self { inner : self . inner . clone () , marker : PhantomData , } } }
};
}
