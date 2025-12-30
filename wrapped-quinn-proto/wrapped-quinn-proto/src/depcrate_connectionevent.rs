// Generated macro for Event (enum)
macro_rules! Depcrate_connectionEvent {
() => {
// Module: crate::connection
// Provides: {"Event"}
// Dependencies: {}
# [doc = " Events of interest to the application"] # [derive (Debug)] pub enum Event { # [doc = " The connection's handshake data is ready"] HandshakeDataReady , # [doc = " The connection was successfully established"] Connected , # [doc = " The connection was lost"] # [doc = ""] # [doc = " Emitted if the peer closes the connection or an error is encountered."] ConnectionLost { # [doc = " Reason that the connection was closed"] reason : ConnectionError , } , # [doc = " Stream events"] Stream (StreamEvent) , # [doc = " One or more application datagrams have been received"] DatagramReceived , # [doc = " One or more application datagrams have been sent after blocking"] DatagramsUnblocked , }
};
}
