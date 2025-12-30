// Generated macro for State (enum)
macro_rules! Depcrate_connection_assemblerState {
() => {
// Module: crate::connection::assembler
// Provides: {"State"}
// Dependencies: {}
# [derive (Debug , Default)] enum State { # [default] Ordered , Unordered { # [doc = " The set of offsets that have been received from the peer, including portions not yet"] # [doc = " read by the application."] recvd : RangeSet , } , }
};
}
