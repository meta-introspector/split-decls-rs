// Generated macro for impl_136 (impl)
macro_rules! Depcrate_client_legacy_connect_httpimpl_136 {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"impl_136"}
// Dependencies: {}
impl < R > tower_service :: Service < Uri > for HttpConnector < R > where R : Resolve + Clone + Send + Sync + 'static , R :: Future : Send , { type Response = TokioIo < TcpStream > ; type Error = ConnectError ; type Future = HttpConnecting < R > ; fn poll_ready (& mut self , cx : & mut task :: Context < '_ >) -> Poll < Result < () , Self :: Error > > { ready ! (self . resolver . poll_ready (cx)) . map_err (ConnectError :: dns) ? ; Poll :: Ready (Ok (())) } fn call (& mut self , dst : Uri) -> Self :: Future { let mut self_ = self . clone () ; HttpConnecting { fut : Box :: pin (async move { self_ . call_async (dst) . await }) , _marker : PhantomData , } } }
};
}
