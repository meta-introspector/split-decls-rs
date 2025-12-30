// Generated macro for impl_144 (impl)
macro_rules! Depcrate_accept_native_tlsimpl_144 {
() => {
// Module: crate::accept::native_tls
// Provides: {"impl_144"}
// Dependencies: {}
impl < IO : ActixStream > AsyncWrite for TlsStream < IO > { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { Pin :: new (& mut * * self . get_mut ()) . poll_write (cx , buf) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Pin :: new (& mut * * self . get_mut ()) . poll_flush (cx) } fn poll_shutdown (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Pin :: new (& mut * * self . get_mut ()) . poll_shutdown (cx) } fn poll_write_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { Pin :: new (& mut * * self . get_mut ()) . poll_write_vectored (cx , bufs) } fn is_write_vectored (& self) -> bool { (* * self) . is_write_vectored () } }
};
}
