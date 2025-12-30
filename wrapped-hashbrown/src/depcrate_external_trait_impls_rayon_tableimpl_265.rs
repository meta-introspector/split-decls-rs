// Generated macro for impl_265 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_tableimpl_265 {
() => {
// Module: crate::external_trait_impls::rayon::table
// Provides: {"impl_265"}
// Dependencies: {}
impl < T > Clone for ParIter < '_ , T > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Self { inner : self . inner . clone () , marker : PhantomData , } } }
};
}
