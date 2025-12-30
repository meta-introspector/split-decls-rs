// Generated macro for impl_168 (impl)
macro_rules! Depcrate_easy_listimpl_168 {
() => {
// Module: crate::easy::list
// Provides: {"impl_168"}
// Dependencies: {}
impl < 'a > fmt :: Debug for Iter < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_list () . entries (self . clone () . map (String :: from_utf8_lossy)) . finish () } }
};
}
