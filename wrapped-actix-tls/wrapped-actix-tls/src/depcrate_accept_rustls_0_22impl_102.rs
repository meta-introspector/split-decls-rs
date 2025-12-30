// Generated macro for impl_102 (impl)
macro_rules! Depcrate_accept_rustls_0_22impl_102 {
() => {
// Module: crate::accept::rustls_0_22
// Provides: {"impl_102"}
// Dependencies: {}
impl < IO : ActixStream > Service < IO > for AcceptorService { type Response = TlsStream < IO > ; type Error = TlsError < io :: Error , Infallible > ; type Future = AcceptFut < IO > ; fn poll_ready (& self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { if self . conns . available (cx) { Poll :: Ready (Ok (())) } else { Poll :: Pending } } fn call (& self , req : IO) -> Self :: Future { AcceptFut { fut : self . acceptor . accept (req) , timeout : sleep (self . handshake_timeout) , _guard : self . conns . get () , } } }
};
}
