// Generated macro for impl_199 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v5_messagesimpl_199 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v5::messages
// Provides: {"impl_199"}
// Dependencies: {}
impl std :: fmt :: Display for Status { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . write_str (match self { Self :: Success => "success" , Self :: GeneralServerFailure => "general server failure" , Self :: ConnectionNotAllowed => "connection not allowed" , Self :: NetworkUnreachable => "network unreachable" , Self :: HostUnreachable => "host unreachable" , Self :: ConnectionRefused => "connection refused" , Self :: TtlExpired => "ttl expired" , Self :: CommandNotSupported => "command not supported" , Self :: AddressTypeNotSupported => "address type not supported" , }) } }
};
}
