// Generated macro for impl_69 (impl)
macro_rules! Depcrate_accept_rustls_0_21impl_69 {
() => {
// Module: crate::accept::rustls_0_21
// Provides: {"impl_69"}
// Dependencies: {}
impl < IO : ActixStream > AsyncRead for TlsStream < IO > { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { Pin :: new (& mut * * self . get_mut ()) . poll_read (cx , buf) } }
};
}
