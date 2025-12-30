// Generated macro for impl_158 (impl)
macro_rules! Depcrate_acceptimpl_158 {
() => {
// Module: crate::accept
// Provides: {"impl_158"}
// Dependencies: {}
impl < TlsErr , SvcErr > fmt :: Display for TlsError < TlsErr , SvcErr > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Timeout => f . write_str ("TLS handshake has timed-out") , Self :: Tls (_) => f . write_str ("TLS handshake error") , Self :: Service (_) => f . write_str ("Service error") , } } }
};
}
