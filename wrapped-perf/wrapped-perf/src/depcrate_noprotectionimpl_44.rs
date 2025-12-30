// Generated macro for impl_44 (impl)
macro_rules! Depcrate_noprotectionimpl_44 {
() => {
// Module: crate::noprotection
// Provides: {"impl_44"}
// Dependencies: {}
impl crypto :: ServerConfig for NoProtectionServerConfig { fn initial_keys (& self , version : u32 , dst_cid : ConnectionId ,) -> Result < crypto :: Keys , crypto :: UnsupportedVersion > { self . inner . initial_keys (version , dst_cid) } fn retry_tag (& self , version : u32 , orig_dst_cid : ConnectionId , packet : & [u8]) -> [u8 ; 16] { self . inner . retry_tag (version , orig_dst_cid , packet) } fn start_session (self : Arc < Self > , version : u32 , params : & transport_parameters :: TransportParameters ,) -> Box < dyn crypto :: Session > { let tls = self . inner . clone () . start_session (version , params) ; Box :: new (NoProtectionSession :: new (tls)) } }
};
}
