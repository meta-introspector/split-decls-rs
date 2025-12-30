// Generated macro for CloseEvent (struct)
macro_rules! Depcrate_websocket_eventsCloseEvent {
() => {
// Module: crate::websocket::events
// Provides: {"CloseEvent"}
// Dependencies: {}
# [doc = " Data emitted by `onclose` event"] # [derive (Clone , Debug)] pub struct CloseEvent { # [doc = " Close code"] pub code : u16 , # [doc = " Close reason"] pub reason : String , # [doc = " If the websockets was closed cleanly"] pub was_clean : bool , }
};
}
