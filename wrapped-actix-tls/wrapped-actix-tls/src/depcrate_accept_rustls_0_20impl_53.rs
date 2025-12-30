// Generated macro for impl_53 (impl)
macro_rules! Depcrate_accept_rustls_0_20impl_53 {
() => {
// Module: crate::accept::rustls_0_20
// Provides: {"impl_53"}
// Dependencies: {}
impl < IO : ActixStream > Future for AcceptFut < IO > { type Output = Result < TlsStream < IO > , TlsError < io :: Error , Infallible > > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; match Pin :: new (& mut this . fut) . poll (cx) { Poll :: Ready (Ok (stream)) => Poll :: Ready (Ok (TlsStream (stream))) , Poll :: Ready (Err (err)) => Poll :: Ready (Err (TlsError :: Tls (err))) , Poll :: Pending => this . timeout . poll (cx) . map (| _ | Err (TlsError :: Timeout)) , } } }
};
}
