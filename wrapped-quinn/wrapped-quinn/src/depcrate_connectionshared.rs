// Generated macro for Shared (struct)
macro_rules! Depcrate_connectionShared {
() => {
// Module: crate::connection
// Provides: {"Shared"}
// Dependencies: {}
# [derive (Debug , Default)] pub (crate) struct Shared { # [doc = " Notified when new streams may be locally initiated due to an increase in stream ID flow"] # [doc = " control budget"] stream_budget_available : [Notify ; 2] , # [doc = " Notified when the peer has initiated a new stream"] stream_incoming : [Notify ; 2] , datagram_received : Notify , datagrams_unblocked : Notify , closed : Notify , }
};
}
