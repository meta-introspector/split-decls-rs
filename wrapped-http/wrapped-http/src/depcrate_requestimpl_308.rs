// Generated macro for impl_308 (impl)
macro_rules! Depcrate_requestimpl_308 {
() => {
// Module: crate::request
// Provides: {"impl_308"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for Request < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Request") . field ("method" , self . method ()) . field ("uri" , self . uri ()) . field ("version" , & self . version ()) . field ("headers" , self . headers ()) . field ("body" , self . body ()) . finish () } }
};
}
