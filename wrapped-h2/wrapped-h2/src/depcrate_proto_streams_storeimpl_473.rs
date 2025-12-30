// Generated macro for impl_473 (impl)
macro_rules! Depcrate_proto_streams_storeimpl_473 {
() => {
// Module: crate::proto::streams::store
// Provides: {"impl_473"}
// Dependencies: {}
impl ops :: IndexMut < Key > for Store { fn index_mut (& mut self , key : Key) -> & mut Self :: Output { self . slab . get_mut (key . index . 0 as usize) . filter (| s | s . id == key . stream_id) . unwrap_or_else (| | { panic ! ("dangling store key for stream_id={:?}" , key . stream_id) ; }) } }
};
}
