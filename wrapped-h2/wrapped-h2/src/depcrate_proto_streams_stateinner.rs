// Generated macro for Inner (enum)
macro_rules! Depcrate_proto_streams_stateInner {
() => {
// Module: crate::proto::streams::state
// Provides: {"Inner"}
// Dependencies: {}
# [derive (Debug , Clone)] enum Inner { Idle , ReservedLocal , ReservedRemote , Open { local : Peer , remote : Peer } , HalfClosedLocal (Peer) , HalfClosedRemote (Peer) , Closed (Cause) , }
};
}
