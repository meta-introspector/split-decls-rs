// Generated macro for impl_71 (impl)
macro_rules! Depcrate_accept_rustls_0_21impl_71 {
() => {
// Module: crate::accept::rustls_0_21
// Provides: {"impl_71"}
// Dependencies: {}
impl < IO : ActixStream > ActixStream for TlsStream < IO > { fn poll_read_ready (& self , cx : & mut Context < '_ >) -> Poll < io :: Result < Ready > > { IO :: poll_read_ready ((* * self) . get_ref () . 0 , cx) } fn poll_write_ready (& self , cx : & mut Context < '_ >) -> Poll < io :: Result < Ready > > { IO :: poll_write_ready ((* * self) . get_ref () . 0 , cx) } }
};
}
