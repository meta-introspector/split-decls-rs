// Generated macro for ClientConfig (struct)
macro_rules! Depcrate_configClientConfig {
() => {
// Module: crate::config
// Provides: {"ClientConfig"}
// Dependencies: {}
# [doc = " Configuration for outgoing connections"] # [doc = ""] # [doc = " Default values should be suitable for most internet applications."] # [derive (Clone)] # [non_exhaustive] pub struct ClientConfig { # [doc = " Transport configuration to use"] pub (crate) transport : Arc < TransportConfig > , # [doc = " Cryptographic configuration to use"] pub (crate) crypto : Arc < dyn crypto :: ClientConfig > , # [doc = " Validation token store to use"] pub (crate) token_store : Arc < dyn TokenStore > , # [doc = " Provider that populates the destination connection ID of Initial Packets"] pub (crate) initial_dst_cid_provider : Arc < dyn Fn () -> ConnectionId + Send + Sync > , # [doc = " QUIC protocol version to use"] pub (crate) version : u32 , }
};
}
