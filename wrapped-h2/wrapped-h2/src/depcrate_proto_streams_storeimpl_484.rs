// Generated macro for impl_484 (impl)
macro_rules! Depcrate_proto_streams_storeimpl_484 {
() => {
// Module: crate::proto::streams::store
// Provides: {"impl_484"}
// Dependencies: {}
impl < 'a > VacantEntry < 'a > { pub fn insert (self , value : Stream) -> Key { let stream_id = value . id ; let index = SlabIndex (self . slab . insert (value) as u32) ; self . ids . insert (index) ; Key { index , stream_id } } }
};
}
