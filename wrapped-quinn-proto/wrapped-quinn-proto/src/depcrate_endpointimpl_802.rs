// Generated macro for impl_802 (impl)
macro_rules! Depcrate_endpointimpl_802 {
() => {
// Module: crate::endpoint
// Provides: {"impl_802"}
// Dependencies: {}
impl fmt :: Debug for Incoming { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Incoming") . field ("addresses" , & self . addresses) . field ("ecn" , & self . ecn) . field ("token" , & self . token) . field ("incoming_idx" , & self . incoming_idx) . finish_non_exhaustive () } }
};
}
