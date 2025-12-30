// Generated macro for impl_77 (impl)
macro_rules! Depcrate_http_requestimpl_77 {
() => {
// Module: crate::http::request
// Provides: {"impl_77"}
// Dependencies: {}
impl fmt :: Debug for Request { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Request") . field ("url" , & self . url ()) . field ("headers" , & self . headers ()) . field ("body_used" , & self . body_used ()) . finish () } }
};
}
