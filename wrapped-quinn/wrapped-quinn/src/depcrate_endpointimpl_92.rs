// Generated macro for impl_92 (impl)
macro_rules! Depcrate_endpointimpl_92 {
() => {
// Module: crate::endpoint
// Provides: {"impl_92"}
// Dependencies: {}
impl fmt :: Debug for RecvState { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("RecvState") . field ("incoming" , & self . incoming) . field ("connections" , & self . connections) . field ("recv_limiter" , & self . recv_limiter) . finish_non_exhaustive () } }
};
}
