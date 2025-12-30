// Generated macro for impl_371 (impl)
macro_rules! Depcrate_connect_native_tlsimpl_371 {
() => {
// Module: crate::connect::native_tls
// Provides: {"impl_371"}
// Dependencies: {}
impl < R : Host , IO > ServiceFactory < Connection < R , IO > > for TlsConnector where IO : ActixStream + 'static , { type Response = Connection < R , AsyncTlsStream < IO > > ; type Error = io :: Error ; type Config = () ; type Service = Self ; type InitError = () ; type Future = Ready < Result < Self :: Service , Self :: InitError > > ; fn new_service (& self , _ : ()) -> Self :: Future { ok (self . clone ()) } }
};
}
