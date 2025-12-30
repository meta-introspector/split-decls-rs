// Generated macro for DEFAULT_TLS_HANDSHAKE_TIMEOUT (const)
macro_rules! Depcrate_acceptDEFAULT_TLS_HANDSHAKE_TIMEOUT {
() => {
// Module: crate::accept
// Provides: {"DEFAULT_TLS_HANDSHAKE_TIMEOUT"}
// Dependencies: {}
# [cfg (any (feature = "openssl" , feature = "rustls-0_20" , feature = "rustls-0_21" , feature = "rustls-0_22" , feature = "rustls-0_23" , feature = "native-tls" ,))] pub (crate) const DEFAULT_TLS_HANDSHAKE_TIMEOUT : std :: time :: Duration = std :: time :: Duration :: from_secs (3) ;
};
}
