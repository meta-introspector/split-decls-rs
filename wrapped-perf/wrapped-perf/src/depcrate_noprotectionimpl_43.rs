// Generated macro for impl_43 (impl)
macro_rules! Depcrate_noprotectionimpl_43 {
() => {
// Module: crate::noprotection
// Provides: {"impl_43"}
// Dependencies: {}
impl crypto :: ClientConfig for NoProtectionClientConfig { fn start_session (self : std :: sync :: Arc < Self > , version : u32 , server_name : & str , params : & transport_parameters :: TransportParameters ,) -> Result < Box < dyn crypto :: Session > , quinn :: ConnectError > { let tls = self . inner . clone () . start_session (version , server_name , params) ? ; Ok (Box :: new (NoProtectionSession :: new (tls))) } }
};
}
