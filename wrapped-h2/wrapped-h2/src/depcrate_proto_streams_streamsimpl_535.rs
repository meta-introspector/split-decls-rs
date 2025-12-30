// Generated macro for impl_535 (impl)
macro_rules! Depcrate_proto_streams_streamsimpl_535 {
() => {
// Module: crate::proto::streams::streams
// Provides: {"impl_535"}
// Dependencies: {}
impl < B , P > Clone for Streams < B , P > where P : Peer , { fn clone (& self) -> Self { self . inner . lock () . unwrap () . refs += 1 ; Streams { inner : self . inner . clone () , send_buffer : self . send_buffer . clone () , _p : :: std :: marker :: PhantomData , } } }
};
}
