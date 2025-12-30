// Generated macro for impl_931 (impl)
macro_rules! Depcrate_http_websocketimpl_931 {
() => {
// Module: crate::http::websocket
// Provides: {"impl_931"}
// Dependencies: {}
impl Stream for Timer { type Item = () ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = & mut * self ; match this . delay . poll_unpin (cx) { Poll :: Ready (_) => { this . delay . reset (this . interval) ; Poll :: Ready (Some (())) } Poll :: Pending => Poll :: Pending , } } }
};
}
