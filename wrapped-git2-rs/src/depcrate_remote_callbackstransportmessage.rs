// Generated macro for TransportMessage (type)
macro_rules! Depcrate_remote_callbacksTransportMessage {
() => {
// Module: crate::remote_callbacks
// Provides: {"TransportMessage"}
// Dependencies: {}
# [doc = " Callback for receiving messages delivered by the transport."] # [doc = ""] # [doc = " The return value indicates whether the network operation should continue."] pub type TransportMessage < 'a > = dyn FnMut (& [u8]) -> bool + 'a ;
};
}
