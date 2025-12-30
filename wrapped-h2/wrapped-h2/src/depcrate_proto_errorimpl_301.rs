// Generated macro for impl_301 (impl)
macro_rules! Depcrate_proto_errorimpl_301 {
() => {
// Module: crate::proto::error
// Provides: {"impl_301"}
// Dependencies: {}
impl Error { pub (crate) fn is_local (& self) -> bool { match * self { Self :: Reset (_ , _ , initiator) | Self :: GoAway (_ , _ , initiator) => initiator . is_local () , Self :: Io (..) => true , } } pub (crate) fn user_go_away (reason : Reason) -> Self { Self :: GoAway (Bytes :: new () , reason , Initiator :: User) } pub (crate) fn library_reset (stream_id : StreamId , reason : Reason) -> Self { Self :: Reset (stream_id , reason , Initiator :: Library) } pub (crate) fn library_go_away (reason : Reason) -> Self { Self :: GoAway (Bytes :: new () , reason , Initiator :: Library) } pub (crate) fn library_go_away_data (reason : Reason , debug_data : impl Into < Bytes >) -> Self { Self :: GoAway (debug_data . into () , reason , Initiator :: Library) } pub (crate) fn remote_reset (stream_id : StreamId , reason : Reason) -> Self { Self :: Reset (stream_id , reason , Initiator :: Remote) } pub (crate) fn remote_go_away (debug_data : Bytes , reason : Reason) -> Self { Self :: GoAway (debug_data , reason , Initiator :: Remote) } }
};
}
