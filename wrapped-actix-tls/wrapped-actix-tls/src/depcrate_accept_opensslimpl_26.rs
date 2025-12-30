// Generated macro for impl_26 (impl)
macro_rules! Depcrate_accept_opensslimpl_26 {
() => {
// Module: crate::accept::openssl
// Provides: {"impl_26"}
// Dependencies: {}
impl < IO : ActixStream > Service < IO > for AcceptorService { type Response = TlsStream < IO > ; type Error = TlsError < Error , Infallible > ; type Future = AcceptFut < IO > ; fn poll_ready (& self , ctx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { if self . conns . available (ctx) { Poll :: Ready (Ok (())) } else { Poll :: Pending } } fn call (& self , io : IO) -> Self :: Future { let ssl_ctx = self . acceptor . context () ; let ssl = Ssl :: new (ssl_ctx) . expect ("Provided SSL acceptor was invalid.") ; AcceptFut { _guard : self . conns . get () , timeout : sleep (self . handshake_timeout) , stream : Some (tokio_openssl :: SslStream :: new (ssl , io) . unwrap ()) , } } }
};
}
