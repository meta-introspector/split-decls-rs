// Generated macro for WebSocketError (enum)
macro_rules! Depcrate_websocketWebSocketError {
() => {
// Module: crate::websocket
// Provides: {"WebSocketError"}
// Dependencies: {}
# [doc = " Error returned by WebSocket"] # [derive (Debug)] # [non_exhaustive] pub enum WebSocketError { # [doc = " The `error` event"] ConnectionError , # [doc = " The `close` event"] ConnectionClose (CloseEvent) , # [doc = " Message failed to send."] MessageSendError (JsError) , }
};
}
