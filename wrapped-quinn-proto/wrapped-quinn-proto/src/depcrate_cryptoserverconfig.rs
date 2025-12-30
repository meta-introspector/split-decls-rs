// Generated macro for ServerConfig (trait)
macro_rules! Depcrate_cryptoServerConfig {
() => {
// Module: crate::crypto
// Provides: {"ServerConfig"}
// Dependencies: {}
# [doc = " Server-side configuration for the crypto protocol"] pub trait ServerConfig : Send + Sync { # [doc = " Create the initial set of keys given the client's initial destination ConnectionId"] fn initial_keys (& self , version : u32 , dst_cid : ConnectionId) -> Result < Keys , UnsupportedVersion > ; # [doc = " Generate the integrity tag for a retry packet"] # [doc = ""] # [doc = " Never called if `initial_keys` rejected `version`."] fn retry_tag (& self , version : u32 , orig_dst_cid : ConnectionId , packet : & [u8]) -> [u8 ; 16] ; # [doc = " Start a server session with this configuration"] # [doc = ""] # [doc = " Never called if `initial_keys` rejected `version`."] fn start_session (self : Arc < Self > , version : u32 , params : & TransportParameters ,) -> Box < dyn Session > ; }
};
}
