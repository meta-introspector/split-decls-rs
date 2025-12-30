// Generated macro for impl_246 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v4impl_246 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v4
// Provides: {"impl_246"}
// Dependencies: {}
impl < C > Service < Uri > for SocksV4 < C > where C : Service < Uri > , C :: Future : Send + 'static , C :: Response : Read + Write + Unpin + Send + 'static , C :: Error : Send + 'static , { type Response = C :: Response ; type Error = SocksError < C :: Error > ; type Future = Handshaking < C :: Future , C :: Response , C :: Error > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) . map_err (SocksError :: Inner) } fn call (& mut self , dst : Uri) -> Self :: Future { let config = self . config . clone () ; let connecting = self . inner . call (config . proxy . clone ()) ; let fut = async move { let port = dst . port () . map (| p | p . as_u16 ()) . unwrap_or (443) ; let host = dst . host () . ok_or (SocksError :: MissingHost) ? . to_string () ; let conn = connecting . await . map_err (SocksError :: Inner) ? ; config . execute (conn , host , port) . await } ; Handshaking { fut : Box :: pin (fut) , _marker : Default :: default () , } } }
};
}
