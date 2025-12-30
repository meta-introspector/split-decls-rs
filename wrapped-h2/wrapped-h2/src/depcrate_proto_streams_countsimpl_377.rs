// Generated macro for impl_377 (impl)
macro_rules! Depcrate_proto_streams_countsimpl_377 {
() => {
// Module: crate::proto::streams::counts
// Provides: {"impl_377"}
// Dependencies: {}
impl Drop for Counts { fn drop (& mut self) { use std :: thread ; if ! thread :: panicking () { debug_assert ! (! self . has_streams ()) ; } } }
};
}
