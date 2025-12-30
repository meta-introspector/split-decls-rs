// Generated macro for impl_148 (impl)
macro_rules! Depcrate_websocket_io_utilimpl_148 {
() => {
// Module: crate::websocket::io_util
// Provides: {"impl_148"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "io-util")))] impl AsyncRead for WebSocket { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { let mut data = if let Some (data) = self . as_mut () . get_mut () . read_pending_bytes . take () { data } else { match ready ! (self . as_mut () . poll_next (cx)) { Some (item) => match try_in_poll_io ! (item) { WebSocketMessage :: Text (s) => s . into_bytes () , WebSocketMessage :: Bytes (data) => data , } , None => return Poll :: Ready (Ok (0)) , } } ; let bytes_to_copy = cmp :: min (buf . len () , data . len ()) ; buf [.. bytes_to_copy] . copy_from_slice (& data [.. bytes_to_copy]) ; if data . len () > bytes_to_copy { data . drain (.. bytes_to_copy) ; self . get_mut () . read_pending_bytes = Some (data) ; } Poll :: Ready (Ok (bytes_to_copy)) } }
};
}
