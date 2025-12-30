// Generated macro for impl_180 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_180 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_180"}
// Dependencies: {}
impl < K : Eq + Hash , V : fmt :: Debug > fmt :: Debug for ParValuesMut < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { ParValues { inner : self . inner . clone () , marker : PhantomData , } . fmt (f) } }
};
}
