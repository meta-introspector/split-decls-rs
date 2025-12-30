// Generated macro for State (enum)
macro_rules! Depcrate_proto_connectionState {
() => {
// Module: crate::proto::connection
// Provides: {"State"}
// Dependencies: {}
# [derive (Debug)] enum State { # [doc = " Currently open in a sane state"] Open , # [doc = " The codec must be flushed"] Closing (Reason , Initiator) , # [doc = " In a closed state"] Closed (Reason , Initiator) , }
};
}
