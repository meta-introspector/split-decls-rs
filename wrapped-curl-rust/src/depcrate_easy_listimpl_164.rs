// Generated macro for impl_164 (impl)
macro_rules! Depcrate_easy_listimpl_164 {
() => {
// Module: crate::easy::list
// Provides: {"impl_164"}
// Dependencies: {}
impl fmt :: Debug for List { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_list () . entries (self . iter () . map (String :: from_utf8_lossy)) . finish () } }
};
}
