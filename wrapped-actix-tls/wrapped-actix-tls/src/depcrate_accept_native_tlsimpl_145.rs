// Generated macro for impl_145 (impl)
macro_rules! Depcrate_accept_native_tlsimpl_145 {
() => {
// Module: crate::accept::native_tls
// Provides: {"impl_145"}
// Dependencies: {}
impl < IO : ActixStream > ActixStream for TlsStream < IO > { fn poll_read_ready (& self , cx : & mut Context < '_ >) -> Poll < io :: Result < Ready > > { IO :: poll_read_ready ((* * self) . get_ref () . get_ref () . get_ref () , cx) } fn poll_write_ready (& self , cx : & mut Context < '_ >) -> Poll < io :: Result < Ready > > { IO :: poll_write_ready ((* * self) . get_ref () . get_ref () . get_ref () , cx) } }
};
}
