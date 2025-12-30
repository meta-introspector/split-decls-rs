// Generated macro for impl_28 (impl)
macro_rules! Depcrate_accept_opensslimpl_28 {
() => {
// Module: crate::accept::openssl
// Provides: {"impl_28"}
// Dependencies: {}
impl < IO : ActixStream > Future for AcceptFut < IO > { type Output = Result < TlsStream < IO > , TlsError < Error , Infallible > > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; match Pin :: new (this . stream . as_mut () . unwrap ()) . poll_accept (cx) { Poll :: Ready (Ok (())) => Poll :: Ready (Ok (this . stream . take () . expect ("Acceptor should not be polled after it has completed.") . into ())) , Poll :: Ready (Err (err)) => Poll :: Ready (Err (TlsError :: Tls (err))) , Poll :: Pending => this . timeout . poll (cx) . map (| _ | Err (TlsError :: Timeout)) , } } }
};
}
