// Generated macro for impl_536 (impl)
macro_rules! Depcrate_proto_streams_streamsimpl_536 {
() => {
// Module: crate::proto::streams::streams
// Provides: {"impl_536"}
// Dependencies: {}
impl < B , P > Drop for Streams < B , P > where P : Peer , { fn drop (& mut self) { if let Ok (mut inner) = self . inner . lock () { inner . refs -= 1 ; if inner . refs == 1 { if let Some (task) = inner . actions . task . take () { task . wake () ; } } } } }
};
}
