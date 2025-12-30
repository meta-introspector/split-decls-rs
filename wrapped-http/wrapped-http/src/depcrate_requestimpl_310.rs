// Generated macro for impl_310 (impl)
macro_rules! Depcrate_requestimpl_310 {
() => {
// Module: crate::request
// Provides: {"impl_310"}
// Dependencies: {}
impl fmt :: Debug for Parts { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Parts") . field ("method" , & self . method) . field ("uri" , & self . uri) . field ("version" , & self . version) . field ("headers" , & self . headers) . finish () } }
};
}
