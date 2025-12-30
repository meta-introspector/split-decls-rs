// Generated macro for impl_146 (impl)
macro_rules! Depcrate_websocket_io_utilimpl_146 {
() => {
// Module: crate::websocket::io_util
// Provides: {"impl_146"}
// Dependencies: {}
impl WebSocket { # [doc = " Returns whether there are pending bytes left after calling [`AsyncRead::poll_read`] on this WebSocket."] # [doc = ""] # [doc = " When calling [`AsyncRead::poll_read`], [`Stream::poll_next`](futures_core::Stream::poll_next) is called"] # [doc = " under the hood, and when the received item is too big to fit into the provided buffer, leftover bytes are"] # [doc = " stored. These leftover bytes are returned by subsequent calls to [`AsyncRead::poll_read`]."] # [cfg_attr (docsrs , doc (cfg (feature = "io-util")))] pub fn has_pending_bytes (& self) -> bool { self . read_pending_bytes . is_some () } }
};
}
