// Generated macro for impl_180 (impl)
macro_rules! Depcrate_rayon_mapimpl_180 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_180"}
// Dependencies: {}
impl < K : fmt :: Debug , V : fmt :: Debug > fmt :: Debug for ParIterMut < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = self . entries . iter () . map (Bucket :: refs) ; f . debug_list () . entries (iter) . finish () } }
};
}
