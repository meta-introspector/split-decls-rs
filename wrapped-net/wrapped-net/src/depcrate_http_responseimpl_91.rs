// Generated macro for impl_91 (impl)
macro_rules! Depcrate_http_responseimpl_91 {
() => {
// Module: crate::http::response
// Provides: {"impl_91"}
// Dependencies: {}
impl fmt :: Debug for Response { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Response") . field ("url" , & self . url ()) . field ("redirected" , & self . redirected ()) . field ("status" , & self . status ()) . field ("headers" , & self . headers ()) . field ("body_used" , & self . body_used ()) . finish_non_exhaustive () } }
};
}
