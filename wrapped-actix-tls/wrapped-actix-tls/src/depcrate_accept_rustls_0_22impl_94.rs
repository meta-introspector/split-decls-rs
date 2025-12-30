// Generated macro for impl_94 (impl)
macro_rules! Depcrate_accept_rustls_0_22impl_94 {
() => {
// Module: crate::accept::rustls_0_22
// Provides: {"impl_94"}
// Dependencies: {}
impl < IO : ActixStream > AsyncRead for TlsStream < IO > { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { Pin :: new (& mut * * self . get_mut ()) . poll_read (cx , buf) } }
};
}
