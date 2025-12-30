// Generated macro for impl_483 (impl)
macro_rules! Depcrate_proto_streams_storeimpl_483 {
() => {
// Module: crate::proto::streams::store
// Provides: {"impl_483"}
// Dependencies: {}
impl < 'a > OccupiedEntry < 'a > { pub fn key (& self) -> Key { let stream_id = * self . ids . key () ; let index = * self . ids . get () ; Key { index , stream_id } } }
};
}
