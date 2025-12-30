// Generated macro for impl_472 (impl)
macro_rules! Depcrate_proto_streams_storeimpl_472 {
() => {
// Module: crate::proto::streams::store
// Provides: {"impl_472"}
// Dependencies: {}
impl ops :: Index < Key > for Store { type Output = Stream ; fn index (& self , key : Key) -> & Self :: Output { self . slab . get (key . index . 0 as usize) . filter (| s | s . id == key . stream_id) . unwrap_or_else (| | { panic ! ("dangling store key for stream_id={:?}" , key . stream_id) ; }) } }
};
}
