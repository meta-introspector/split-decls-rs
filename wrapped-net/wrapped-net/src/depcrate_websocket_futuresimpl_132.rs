// Generated macro for impl_132 (impl)
macro_rules! Depcrate_websocket_futuresimpl_132 {
() => {
// Module: crate::websocket::futures
// Provides: {"impl_132"}
// Dependencies: {}
impl Sink < Message > for WebSocket { type Error = WebSocketError ; fn poll_ready (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { let ready_state = self . ws . ready_state () ; if ready_state == 0 { * self . sink_waker . borrow_mut () = Some (cx . waker () . clone ()) ; Poll :: Pending } else { Poll :: Ready (Ok (())) } } fn start_send (self : Pin < & mut Self > , item : Message) -> Result < () , Self :: Error > { let result = match item { Message :: Bytes (bytes) => self . ws . send_with_u8_array (& bytes) , Message :: Text (message) => self . ws . send_with_str (& message) , } ; match result { Ok (_) => Ok (()) , Err (e) => Err (WebSocketError :: MessageSendError (js_to_js_error (e))) , } } fn poll_flush (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } fn poll_close (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } }
};
}
