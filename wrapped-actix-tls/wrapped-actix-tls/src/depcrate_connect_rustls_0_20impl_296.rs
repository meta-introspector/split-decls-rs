// Generated macro for impl_296 (impl)
macro_rules! Depcrate_connect_rustls_0_20impl_296 {
() => {
// Module: crate::connect::rustls_0_20
// Provides: {"impl_296"}
// Dependencies: {}
impl < R , IO > Service < Connection < R , IO > > for TlsConnectorService where R : Host , IO : ActixStream , { type Response = Connection < R , AsyncTlsStream < IO > > ; type Error = io :: Error ; type Future = ConnectFut < R , IO > ; actix_service :: always_ready ! () ; fn call (& self , connection : Connection < R , IO >) -> Self :: Future { tracing :: trace ! ("TLS handshake start for: {:?}" , connection . hostname ()) ; let (stream , connection) = connection . replace_io (()) ; match ServerName :: try_from (connection . hostname ()) { Ok (host) => ConnectFut :: Future { connect : RustlsTlsConnector :: from (Arc :: clone (& self . connector)) . connect (host , stream) , connection : Some (connection) , } , Err (_) => ConnectFut :: InvalidDns , } } }
};
}
