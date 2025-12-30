// Generated macro for try_in_poll_io (macro)
macro_rules! Depcrate_websocket_io_utiltry_in_poll_io {
() => {
// Module: crate::websocket::io_util
// Provides: {"try_in_poll_io"}
// Dependencies: {}
macro_rules ! try_in_poll_io { ($ expr : expr) => { { match $ expr { Ok (o) => o , Err (WebSocketError :: ConnectionClose (event)) if event . was_clean => { return Poll :: Ready (Ok (0)) ; } Err (e) => return Poll :: Ready (Err (io :: Error :: new (io :: ErrorKind :: Other , e))) , } } } ; }
};
}
