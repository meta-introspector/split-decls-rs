// Generated macro for ConnectionEventInner (enum)
macro_rules! Depcrate_sharedConnectionEventInner {
() => {
// Module: crate::shared
// Provides: {"ConnectionEventInner"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum ConnectionEventInner { # [doc = " A datagram has been received for the Connection"] Datagram (DatagramConnectionEvent) , # [doc = " New connection identifiers have been issued for the Connection"] NewIdentifiers (Vec < IssuedCid > , Instant) , }
};
}
