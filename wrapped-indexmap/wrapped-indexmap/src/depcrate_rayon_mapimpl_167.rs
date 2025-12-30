// Generated macro for impl_167 (impl)
macro_rules! Depcrate_rayon_mapimpl_167 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_167"}
// Dependencies: {}
impl < K : fmt :: Debug , V : fmt :: Debug > fmt :: Debug for IntoParIter < K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = self . entries . iter () . map (Bucket :: refs) ; f . debug_list () . entries (iter) . finish () } }
};
}
