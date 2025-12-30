// Generated macro for impl_233 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v4_messagesimpl_233 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v4::messages
// Provides: {"impl_233"}
// Dependencies: {}
impl std :: fmt :: Display for Status { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . write_str (match self { Self :: Success => "success" , Self :: Failed => "server failed to execute command" , Self :: IdentFailure => "server ident service failed" , Self :: IdentMismatch => "server ident service did not recognise client identifier" , }) } }
};
}
