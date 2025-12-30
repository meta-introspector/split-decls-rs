// Generated macro for impl_143 (impl)
macro_rules! Depcrate_accept_native_tlsimpl_143 {
() => {
// Module: crate::accept::native_tls
// Provides: {"impl_143"}
// Dependencies: {}
impl < IO : ActixStream > AsyncRead for TlsStream < IO > { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { Pin :: new (& mut * * self . get_mut ()) . poll_read (cx , buf) } }
};
}
