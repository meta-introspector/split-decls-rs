// Generated macro for impl_177 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_177 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_177"}
// Dependencies: {}
impl < K : fmt :: Debug + Eq + Hash , V : fmt :: Debug > fmt :: Debug for ParIterMut < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { ParIter { inner : self . inner . clone () , marker : PhantomData , } . fmt (f) } }
};
}
