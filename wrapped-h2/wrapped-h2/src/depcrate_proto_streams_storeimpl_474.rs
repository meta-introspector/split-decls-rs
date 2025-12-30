// Generated macro for impl_474 (impl)
macro_rules! Depcrate_proto_streams_storeimpl_474 {
() => {
// Module: crate::proto::streams::store
// Provides: {"impl_474"}
// Dependencies: {}
impl Store { # [cfg (feature = "unstable")] pub fn num_active_streams (& self) -> usize { self . ids . len () } # [cfg (feature = "unstable")] pub fn num_wired_streams (& self) -> usize { self . slab . len () } }
};
}
