// Generated macro for WebSocket (struct)
macro_rules! Depcrate_websocket_futuresWebSocket {
() => {
// Module: crate::websocket::futures
// Provides: {"WebSocket"}
// Dependencies: {}
# [doc = " Wrapper around browser's WebSocket API."] # [allow (missing_debug_implementations)] # [pin_project (PinnedDrop)] pub struct WebSocket { ws : web_sys :: WebSocket , sink_waker : Rc < RefCell < Option < Waker > > > , # [pin] message_receiver : mpsc :: UnboundedReceiver < StreamMessage > , # [allow (clippy :: type_complexity)] closures : (Closure < dyn FnMut () > , Closure < dyn FnMut (MessageEvent) > , Closure < dyn FnMut (web_sys :: Event) > , Closure < dyn FnMut (web_sys :: CloseEvent) > ,) , # [doc = " Leftover bytes when using `AsyncRead`."] # [doc = ""] # [doc = " These bytes are drained and returned in subsequent calls to `poll_read`."] # [cfg (feature = "io-util")] pub (super) read_pending_bytes : Option < Vec < u8 > > , }
};
}
