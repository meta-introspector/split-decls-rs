// Generated macro for impl_866 (impl)
macro_rules! Depcrate_serverimpl_866 {
() => {
// Module: crate::server
// Provides: {"impl_866"}
// Dependencies: {}
impl < T , B > fmt :: Debug for Handshake < T , B > where T : AsyncRead + AsyncWrite + fmt :: Debug , B : fmt :: Debug + Buf , { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { write ! (fmt , "server::Handshake") } }
};
}
