// Generated macro for impl_173 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_173 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_173"}
// Dependencies: {}
impl < K , V > Clone for ParValues < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Self { inner : self . inner . clone () , marker : PhantomData , } } }
};
}
