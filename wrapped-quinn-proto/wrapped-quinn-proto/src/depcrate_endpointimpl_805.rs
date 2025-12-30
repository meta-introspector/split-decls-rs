// Generated macro for impl_805 (impl)
macro_rules! Depcrate_endpointimpl_805 {
() => {
// Module: crate::endpoint
// Provides: {"impl_805"}
// Dependencies: {}
impl Drop for IncomingImproperDropWarner { fn drop (& mut self) { warn ! ("quinn_proto::Incoming dropped without passing to Endpoint::accept/refuse/retry/ignore \
               (may cause memory leak and eventual inability to accept new connections)") ; } }
};
}
