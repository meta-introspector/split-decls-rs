// Generated macro for EndpointConfig (struct)
macro_rules! Depcrate_configEndpointConfig {
() => {
// Module: crate::config
// Provides: {"EndpointConfig"}
// Dependencies: {}
# [doc = " Global configuration for the endpoint, affecting all connections"] # [doc = ""] # [doc = " Default values should be suitable for most internet applications."] # [derive (Clone)] pub struct EndpointConfig { pub (crate) reset_key : Arc < dyn HmacKey > , pub (crate) max_udp_payload_size : VarInt , # [doc = " CID generator factory"] # [doc = ""] # [doc = " Create a cid generator for local cid in Endpoint struct"] pub (crate) connection_id_generator_factory : Arc < dyn Fn () -> Box < dyn ConnectionIdGenerator > + Send + Sync > , pub (crate) supported_versions : Vec < u32 > , pub (crate) grease_quic_bit : bool , # [doc = " Minimum interval between outgoing stateless reset packets"] pub (crate) min_reset_interval : Duration , # [doc = " Optional seed to be used internally for random number generation"] pub (crate) rng_seed : Option < [u8 ; 32] > , }
};
}
