// Generated macro for impl_269 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_tableimpl_269 {
() => {
// Module: crate::external_trait_impls::rayon::table
// Provides: {"impl_269"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for ParIterMut < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { ParIter { inner : self . inner . clone () , marker : PhantomData , } . fmt (f) } }
};
}
