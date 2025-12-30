// Generated macro for impl_558 (impl)
macro_rules! Depcrate_connection_streams_stateimpl_558 {
() => {
// Module: crate::connection::streams::state
// Provides: {"impl_558"}
// Dependencies: {}
impl StreamRecv { # [doc = " Returns a reference to the inner `Recv` if the stream is open"] pub (super) fn as_open_recv (& self) -> Option < & Recv > { match self { Self :: Open (r) => Some (r) , _ => None , } } pub (super) fn as_open_recv_mut (& mut self) -> Option < & mut Recv > { match self { Self :: Open (r) => Some (r) , _ => None , } } pub (super) fn into_inner (self) -> Box < Recv > { match self { Self :: Free (r) | Self :: Open (r) => r , } } pub (super) fn free (self , initial_max_data : u64) -> Self { match self { Self :: Free (_) => unreachable ! ("Self::Free on reinit()") , Self :: Open (mut recv) => { recv . reinit (initial_max_data) ; Self :: Free (recv) } } } }
};
}
