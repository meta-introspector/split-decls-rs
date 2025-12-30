// Generated macro for Message (enum)
macro_rules! Depcrate_websocketMessage {
() => {
// Module: crate::websocket
// Provides: {"Message"}
// Dependencies: {}
# [doc = " Message sent to and received from WebSocket."] # [derive (Debug , PartialEq , Eq , Clone)] pub enum Message { # [doc = " String message"] Text (String) , # [doc = " ArrayBuffer parsed into bytes"] Bytes (Vec < u8 >) , }
};
}
