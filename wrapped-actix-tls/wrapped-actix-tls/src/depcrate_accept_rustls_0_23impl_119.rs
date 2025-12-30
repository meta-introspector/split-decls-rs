// Generated macro for impl_119 (impl)
macro_rules! Depcrate_accept_rustls_0_23impl_119 {
() => {
// Module: crate::accept::rustls_0_23
// Provides: {"impl_119"}
// Dependencies: {}
impl < IO : ActixStream > AsyncRead for TlsStream < IO > { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { Pin :: new (& mut * * self . get_mut ()) . poll_read (cx , buf) } }
};
}
