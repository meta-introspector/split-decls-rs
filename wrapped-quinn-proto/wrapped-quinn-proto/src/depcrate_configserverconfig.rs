// Generated macro for ServerConfig (struct)
macro_rules! Depcrate_configServerConfig {
() => {
// Module: crate::config
// Provides: {"ServerConfig"}
// Dependencies: {}
# [doc = " Parameters governing incoming connections"] # [doc = ""] # [doc = " Default values should be suitable for most internet applications."] # [derive (Clone)] pub struct ServerConfig { # [doc = " Transport configuration to use for incoming connections"] pub transport : Arc < TransportConfig > , # [doc = " TLS configuration used for incoming connections"] # [doc = ""] # [doc = " Must be set to use TLS 1.3 only."] pub crypto : Arc < dyn crypto :: ServerConfig > , # [doc = " Configuration for sending and handling validation tokens"] pub validation_token : ValidationTokenConfig , # [doc = " Used to generate one-time AEAD keys to protect handshake tokens"] pub (crate) token_key : Arc < dyn HandshakeTokenKey > , # [doc = " Duration after a retry token was issued for which it's considered valid"] pub (crate) retry_token_lifetime : Duration , # [doc = " Whether to allow clients to migrate to new addresses"] # [doc = ""] # [doc = " Improves behavior for clients that move between different internet connections or suffer NAT"] # [doc = " rebinding. Enabled by default."] pub (crate) migration : bool , pub (crate) preferred_address_v4 : Option < SocketAddrV4 > , pub (crate) preferred_address_v6 : Option < SocketAddrV6 > , pub (crate) max_incoming : usize , pub (crate) incoming_buffer_size : u64 , pub (crate) incoming_buffer_size_total : u64 , pub (crate) time_source : Arc < dyn TimeSource > , }
};
}
