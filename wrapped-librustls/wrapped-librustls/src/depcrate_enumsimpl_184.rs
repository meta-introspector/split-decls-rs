// Generated macro for impl_184 (impl)
macro_rules! Depcrate_enumsimpl_184 {
() => {
// Module: crate::enums
// Provides: {"impl_184"}
// Dependencies: {}
impl From < & SupportedProtocolVersion > for rustls_tls_version { fn from (version : & SupportedProtocolVersion) -> Self { match version . version { ProtocolVersion :: SSLv2 => rustls_tls_version :: Sslv2 , ProtocolVersion :: SSLv3 => rustls_tls_version :: Sslv3 , ProtocolVersion :: TLSv1_0 => rustls_tls_version :: Tlsv1_0 , ProtocolVersion :: TLSv1_1 => rustls_tls_version :: Tlsv1_1 , ProtocolVersion :: TLSv1_2 => rustls_tls_version :: Tlsv1_2 , ProtocolVersion :: TLSv1_3 => rustls_tls_version :: Tlsv1_3 , _ => rustls_tls_version :: Unknown , } } }
};
}
