// Generated macro for impl_204 (impl)
macro_rules! Depcrate_rayon_mapimpl_204 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_204"}
// Dependencies: {}
impl < K , V : fmt :: Debug > fmt :: Debug for ParValuesMut < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = self . entries . iter () . map (Bucket :: value_ref) ; f . debug_list () . entries (iter) . finish () } }
};
}
