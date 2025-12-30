// Generated macro for impl_654 (impl)
macro_rules! Depcrate_configimpl_654 {
() => {
// Module: crate::config
// Provides: {"impl_654"}
// Dependencies: {}
impl fmt :: Debug for EndpointConfig { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("EndpointConfig") . field ("max_udp_payload_size" , & self . max_udp_payload_size) . field ("supported_versions" , & self . supported_versions) . field ("grease_quic_bit" , & self . grease_quic_bit) . field ("rng_seed" , & self . rng_seed) . finish_non_exhaustive () } }
};
}
