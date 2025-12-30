// Generated macro for impl_192 (impl)
macro_rules! Depcrate_rayon_mapimpl_192 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_192"}
// Dependencies: {}
impl < K : fmt :: Debug , V > fmt :: Debug for ParKeys < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = self . entries . iter () . map (Bucket :: key_ref) ; f . debug_list () . entries (iter) . finish () } }
};
}
