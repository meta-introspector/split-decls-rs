// Generated macro for impl_227 (impl)
macro_rules! Depcrate_rayon_setimpl_227 {
() => {
// Module: crate::rayon::set
// Provides: {"impl_227"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for IntoParIter < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = self . entries . iter () . map (Bucket :: key_ref) ; f . debug_list () . entries (iter) . finish () } }
};
}
