// Generated macro for Connecting (struct)
macro_rules! Depcrate_connectionConnecting {
() => {
// Module: crate::connection
// Provides: {"Connecting"}
// Dependencies: {}
# [doc = " In-progress connection attempt future"] # [derive (Debug)] pub struct Connecting { conn : Option < ConnectionRef > , connected : oneshot :: Receiver < bool > , handshake_data_ready : Option < oneshot :: Receiver < () > > , }
};
}
