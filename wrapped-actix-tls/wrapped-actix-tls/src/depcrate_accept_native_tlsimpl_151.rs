// Generated macro for impl_151 (impl)
macro_rules! Depcrate_accept_native_tlsimpl_151 {
() => {
// Module: crate::accept::native_tls
// Provides: {"impl_151"}
// Dependencies: {}
impl < IO : ActixStream + 'static > Service < IO > for AcceptorService { type Response = TlsStream < IO > ; type Error = TlsError < Error , Infallible > ; type Future = LocalBoxFuture < 'static , Result < Self :: Response , Self :: Error > > ; fn poll_ready (& self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { if self . conns . available (cx) { Poll :: Ready (Ok (())) } else { Poll :: Pending } } fn call (& self , io : IO) -> Self :: Future { let guard = self . conns . get () ; let acceptor = self . acceptor . clone () ; let dur = self . handshake_timeout ; Box :: pin (async move { match timeout (dur , acceptor . accept (io)) . await { Ok (Ok (io)) => { drop (guard) ; Ok (TlsStream (io)) } Ok (Err (err)) => Err (TlsError :: Tls (err)) , Err (_timeout) => Err (TlsError :: Timeout) , } }) } }
};
}
