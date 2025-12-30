// Generated macro for ConnectionSet (struct)
macro_rules! Depcrate_endpointConnectionSet {
() => {
// Module: crate::endpoint
// Provides: {"ConnectionSet"}
// Dependencies: {}
# [derive (Debug)] struct ConnectionSet { # [doc = " Senders for communicating with the endpoint's connections"] senders : FxHashMap < ConnectionHandle , mpsc :: UnboundedSender < ConnectionEvent > > , # [doc = " Stored to give out clones to new ConnectionInners"] sender : mpsc :: UnboundedSender < (ConnectionHandle , EndpointEvent) > , # [doc = " Set if the endpoint has been manually closed"] close : Option < (VarInt , Bytes) > , }
};
}
