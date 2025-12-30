// Generated macro for impl_18 (impl)
macro_rules! Depcrate_accept_opensslimpl_18 {
() => {
// Module: crate::accept::openssl
// Provides: {"impl_18"}
// Dependencies: {}
impl < IO : ActixStream > AsyncRead for TlsStream < IO > { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { Pin :: new (& mut * * self . get_mut ()) . poll_read (cx , buf) } }
};
}
