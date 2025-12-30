// Generated macro for impl_336 (impl)
macro_rules! Depcrate_connect_rustls_0_22impl_336 {
() => {
// Module: crate::connect::rustls_0_22
// Provides: {"impl_336"}
// Dependencies: {}
impl < R , IO > Service < Connection < R , IO > > for TlsConnectorService where R : Host , IO : ActixStream , { type Response = Connection < R , AsyncTlsStream < IO > > ; type Error = io :: Error ; type Future = ConnectFut < R , IO > ; actix_service :: always_ready ! () ; fn call (& self , connection : Connection < R , IO >) -> Self :: Future { tracing :: trace ! ("TLS handshake start for: {:?}" , connection . hostname ()) ; let (stream , conn) = connection . replace_io (()) ; match ServerName :: try_from (conn . hostname ()) { Ok (host) => ConnectFut :: Future { connect : RustlsTlsConnector :: from (Arc :: clone (& self . connector)) . connect (host . to_owned () , stream) , connection : Some (conn) , } , Err (_) => ConnectFut :: InvalidServerName , } } }
};
}
