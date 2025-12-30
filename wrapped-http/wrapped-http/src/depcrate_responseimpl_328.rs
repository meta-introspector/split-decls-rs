// Generated macro for impl_328 (impl)
macro_rules! Depcrate_responseimpl_328 {
() => {
// Module: crate::response
// Provides: {"impl_328"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for Response < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Response") . field ("status" , & self . status ()) . field ("version" , & self . version ()) . field ("headers" , self . headers ()) . field ("body" , self . body ()) . finish () } }
};
}
