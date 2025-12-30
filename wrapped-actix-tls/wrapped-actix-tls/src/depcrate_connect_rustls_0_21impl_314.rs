// Generated macro for impl_314 (impl)
macro_rules! Depcrate_connect_rustls_0_21impl_314 {
() => {
// Module: crate::connect::rustls_0_21
// Provides: {"impl_314"}
// Dependencies: {}
impl < R , IO > ServiceFactory < Connection < R , IO > > for TlsConnector where R : Host , IO : ActixStream + 'static , { type Response = Connection < R , AsyncTlsStream < IO > > ; type Error = io :: Error ; type Config = () ; type Service = TlsConnectorService ; type InitError = () ; type Future = Ready < Result < Self :: Service , Self :: InitError > > ; fn new_service (& self , _ : ()) -> Self :: Future { ok (TlsConnectorService { connector : self . connector . clone () , }) } }
};
}
