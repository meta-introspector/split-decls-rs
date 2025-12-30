// Generated macro for impl_174 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_174 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_174"}
// Dependencies: {}
impl < K : Eq + Hash , V : fmt :: Debug > fmt :: Debug for ParValues < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = unsafe { self . inner . iter () } . map (| x | unsafe { & x . as_ref () . 1 }) ; f . debug_list () . entries (iter) . finish () } }
};
}
