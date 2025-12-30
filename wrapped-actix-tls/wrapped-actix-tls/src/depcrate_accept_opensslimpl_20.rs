// Generated macro for impl_20 (impl)
macro_rules! Depcrate_accept_opensslimpl_20 {
() => {
// Module: crate::accept::openssl
// Provides: {"impl_20"}
// Dependencies: {}
impl < IO : ActixStream > ActixStream for TlsStream < IO > { fn poll_read_ready (& self , cx : & mut Context < '_ >) -> Poll < io :: Result < Ready > > { IO :: poll_read_ready ((* * self) . get_ref () , cx) } fn poll_write_ready (& self , cx : & mut Context < '_ >) -> Poll < io :: Result < Ready > > { IO :: poll_write_ready ((* * self) . get_ref () , cx) } }
};
}
