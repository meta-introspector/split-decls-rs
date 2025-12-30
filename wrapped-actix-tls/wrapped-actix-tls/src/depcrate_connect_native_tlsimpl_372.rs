// Generated macro for impl_372 (impl)
macro_rules! Depcrate_connect_native_tlsimpl_372 {
() => {
// Module: crate::connect::native_tls
// Provides: {"impl_372"}
// Dependencies: {}
# [doc = " The `native-tls` connector is both it's ServiceFactory and Service impl type."] # [doc = " As the factory and service share the same type and state."] impl < R , IO > Service < Connection < R , IO > > for TlsConnector where R : Host , IO : ActixStream + 'static , { type Response = Connection < R , AsyncTlsStream < IO > > ; type Error = io :: Error ; type Future = LocalBoxFuture < 'static , Result < Self :: Response , Self :: Error > > ; actix_service :: always_ready ! () ; fn call (& self , stream : Connection < R , IO >) -> Self :: Future { let (io , stream) = stream . replace_io (()) ; let connector = self . connector . clone () ; Box :: pin (async move { trace ! ("TLS handshake start for: {:?}" , stream . hostname ()) ; connector . connect (stream . hostname () , io) . await . map (| res | { trace ! ("TLS handshake success: {:?}" , stream . hostname ()) ; stream . replace_io (res) . 1 }) . map_err (| err | { trace ! ("TLS handshake error: {err:?}") ; io :: Error :: other (format ! ("{err}")) }) }) } }
};
}
