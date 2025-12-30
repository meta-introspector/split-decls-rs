// Generated macro for impl_789 (impl)
macro_rules! Depcrate_endpointimpl_789 {
() => {
// Module: crate::endpoint
// Provides: {"impl_789"}
// Dependencies: {}
impl fmt :: Debug for Endpoint { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("Endpoint") . field ("rng" , & self . rng) . field ("index" , & self . index) . field ("connections" , & self . connections) . field ("config" , & self . config) . field ("server_config" , & self . server_config) . field ("incoming_buffers.len" , & self . incoming_buffers . len ()) . field ("all_incoming_buffers_total_bytes" , & self . all_incoming_buffers_total_bytes ,) . finish () } }
};
}
