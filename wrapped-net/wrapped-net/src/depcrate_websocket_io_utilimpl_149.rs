// Generated macro for impl_149 (impl)
macro_rules! Depcrate_websocket_io_utilimpl_149 {
() => {
// Module: crate::websocket::io_util
// Provides: {"impl_149"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "io-util")))] impl AsyncWrite for WebSocket { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { let _ = AsyncWrite :: poll_flush (self . as_mut () , cx) ; try_in_poll_io ! (ready ! (self . as_mut () . poll_ready (cx))) ; try_in_poll_io ! (self . start_send (WebSocketMessage :: Bytes (buf . to_vec ()))) ; Poll :: Ready (Ok (buf . len ())) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { let res = ready ! (Sink :: poll_flush (self , cx)) ; Poll :: Ready (ws_result_to_io_result (res)) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { let res = ready ! (Sink :: poll_close (self , cx)) ; Poll :: Ready (ws_result_to_io_result (res)) } }
};
}
