// Generated macro for impl_169 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_169 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_169"}
// Dependencies: {}
impl < K , V > Clone for ParKeys < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Self { inner : self . inner . clone () , marker : PhantomData , } } }
};
}
