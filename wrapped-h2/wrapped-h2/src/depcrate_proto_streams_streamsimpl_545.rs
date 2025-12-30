// Generated macro for impl_545 (impl)
macro_rules! Depcrate_proto_streams_streamsimpl_545 {
() => {
// Module: crate::proto::streams::streams
// Provides: {"impl_545"}
// Dependencies: {}
impl < B > SendBuffer < B > { fn new () -> Self { let inner = Mutex :: new (Buffer :: new ()) ; SendBuffer { inner } } pub fn is_empty (& self) -> bool { let buf = self . inner . lock () . unwrap () ; buf . is_empty () } }
};
}
