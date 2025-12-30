// Generated macro for ClientConfig (trait)
macro_rules! Depcrate_cryptoClientConfig {
() => {
// Module: crate::crypto
// Provides: {"ClientConfig"}
// Dependencies: {}
# [doc = " Client-side configuration for the crypto protocol"] pub trait ClientConfig : Send + Sync { # [doc = " Start a client session with this configuration"] fn start_session (self : Arc < Self > , version : u32 , server_name : & str , params : & TransportParameters ,) -> Result < Box < dyn Session > , ConnectError > ; }
};
}
