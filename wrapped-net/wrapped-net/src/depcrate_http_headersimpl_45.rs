// Generated macro for impl_45 (impl)
macro_rules! Depcrate_http_headersimpl_45 {
() => {
// Module: crate::http::headers
// Provides: {"impl_45"}
// Dependencies: {}
impl fmt :: Debug for Headers { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut dbg = f . debug_struct ("Headers") ; for (key , value) in self . entries () { dbg . field (& key , & value) ; } dbg . finish () } }
};
}
