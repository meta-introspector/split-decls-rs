// Generated macro for impl_166 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_166 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_166"}
// Dependencies: {}
impl < K : fmt :: Debug + Eq + Hash , V : fmt :: Debug > fmt :: Debug for ParIter < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = unsafe { self . inner . iter () } . map (| x | unsafe { let r = x . as_ref () ; (& r . 0 , & r . 1) }) ; f . debug_list () . entries (iter) . finish () } }
};
}
