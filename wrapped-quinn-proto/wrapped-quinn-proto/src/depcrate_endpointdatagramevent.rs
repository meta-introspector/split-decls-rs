// Generated macro for DatagramEvent (enum)
macro_rules! Depcrate_endpointDatagramEvent {
() => {
// Module: crate::endpoint
// Provides: {"DatagramEvent"}
// Dependencies: {}
# [doc = " Event resulting from processing a single datagram"] pub enum DatagramEvent { # [doc = " The datagram is redirected to its `Connection`"] ConnectionEvent (ConnectionHandle , ConnectionEvent) , # [doc = " The datagram may result in starting a new `Connection`"] NewConnection (Incoming) , # [doc = " Response generated directly by the endpoint"] Response (Transmit) , }
};
}
