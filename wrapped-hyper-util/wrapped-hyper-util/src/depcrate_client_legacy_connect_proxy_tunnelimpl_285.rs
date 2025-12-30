// Generated macro for impl_285 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_tunnelimpl_285 {
() => {
// Module: crate::client::legacy::connect::proxy::tunnel
// Provides: {"impl_285"}
// Dependencies: {}
impl < C > Service < Uri > for Tunnel < C > where C : Service < Uri > , C :: Future : Send + 'static , C :: Response : Read + Write + Unpin + Send + 'static , C :: Error : Into < Box < dyn StdError + Send + Sync > > , { type Response = C :: Response ; type Error = TunnelError ; type Future = Tunneling < C :: Future , C :: Response > ; fn poll_ready (& mut self , cx : & mut task :: Context < '_ >) -> Poll < Result < () , Self :: Error > > { ready ! (self . inner . poll_ready (cx)) . map_err (| e | TunnelError :: ConnectFailed (e . into ())) ? ; Poll :: Ready (Ok (())) } fn call (& mut self , dst : Uri) -> Self :: Future { let connecting = self . inner . call (self . proxy_dst . clone ()) ; let headers = self . headers . clone () ; Tunneling { fut : Box :: pin (async move { let conn = connecting . await . map_err (| e | TunnelError :: ConnectFailed (e . into ())) ? ; tunnel (conn , dst . host () . ok_or (TunnelError :: MissingHost) ? , dst . port () . map (| p | p . as_u16 ()) . unwrap_or (443) , & headers ,) . await }) , _marker : PhantomData , } } }
};
}
