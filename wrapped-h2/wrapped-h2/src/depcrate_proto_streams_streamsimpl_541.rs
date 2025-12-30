// Generated macro for impl_541 (impl)
macro_rules! Depcrate_proto_streams_streamsimpl_541 {
() => {
// Module: crate::proto::streams::streams
// Provides: {"impl_541"}
// Dependencies: {}
impl Clone for OpaqueStreamRef { fn clone (& self) -> Self { let mut inner = self . inner . lock () . unwrap () ; inner . store . resolve (self . key) . ref_inc () ; inner . refs += 1 ; OpaqueStreamRef { inner : self . inner . clone () , key : self . key , } } }
};
}
