// Generated macro for impl_234 (impl)
macro_rules! Depcrate_rayon_setimpl_234 {
() => {
// Module: crate::rayon::set
// Provides: {"impl_234"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for ParIter < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = self . entries . iter () . map (Bucket :: key_ref) ; f . debug_list () . entries (iter) . finish () } }
};
}
