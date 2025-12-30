// Generated macro for impl_277 (impl)
macro_rules! Depcrate_connect_opensslimpl_277 {
() => {
// Module: crate::connect::openssl
// Provides: {"impl_277"}
// Dependencies: {}
impl < R , IO > Service < Connection < R , IO > > for TlsConnectorService where R : Host , IO : ActixStream , { type Response = Connection < R , AsyncSslStream < IO > > ; type Error = io :: Error ; type Future = ConnectFut < R , IO > ; actix_service :: always_ready ! () ; fn call (& self , stream : Connection < R , IO >) -> Self :: Future { trace ! ("TLS handshake start for: {:?}" , stream . hostname ()) ; let (io , stream) = stream . replace_io (()) ; let host = stream . hostname () ; let config = self . connector . configure () . expect ("SSL connect configuration was invalid.") ; let ssl = config . into_ssl (host) . expect ("SSL connect configuration was invalid.") ; ConnectFut { io : Some (AsyncSslStream :: new (ssl , io) . unwrap ()) , stream : Some (stream) , } } }
};
}
