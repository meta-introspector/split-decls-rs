// Generated macro for impl_128 (impl)
macro_rules! Depcrate_setimpl_128 {
() => {
// Module: crate::set
// Provides: {"impl_128"}
// Dependencies: {}
impl < T , S > fmt :: Debug for IndexSet < T , S > where T : fmt :: Debug , { # [cfg (not (feature = "test_debug"))] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self . iter ()) . finish () } # [cfg (feature = "test_debug")] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("IndexSet") . field ("map" , & self . map) . finish () } }
};
}
