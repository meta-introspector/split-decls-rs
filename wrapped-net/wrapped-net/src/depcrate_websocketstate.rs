// Generated macro for State (enum)
macro_rules! Depcrate_websocketState {
() => {
// Module: crate::websocket
// Provides: {"State"}
// Dependencies: {}
# [doc = " The state of the websocket."] # [doc = ""] # [doc = " See [`WebSocket.readyState` on MDN](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/readyState)"] # [doc = " to learn more."] # [derive (Copy , Clone , Debug)] pub enum State { # [doc = " The connection has not yet been established."] Connecting , # [doc = " The WebSocket connection is established and communication is possible."] Open , # [doc = " The connection is going through the closing handshake, or the close() method has been"] # [doc = " invoked."] Closing , # [doc = " The connection has been closed or could not be opened."] Closed , }
};
}
