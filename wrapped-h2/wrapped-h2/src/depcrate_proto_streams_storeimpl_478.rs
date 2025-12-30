// Generated macro for impl_478 (impl)
macro_rules! Depcrate_proto_streams_storeimpl_478 {
() => {
// Module: crate::proto::streams::store
// Provides: {"impl_478"}
// Dependencies: {}
impl < 'a > Ptr < 'a > { # [doc = " Returns the Key associated with the stream"] pub fn key (& self) -> Key { self . key } pub fn store_mut (& mut self) -> & mut Store { self . store } # [doc = " Remove the stream from the store"] pub fn remove (self) -> StreamId { debug_assert ! (! self . store . ids . contains_key (& self . key . stream_id)) ; let stream = self . store . slab . remove (self . key . index . 0 as usize) ; assert_eq ! (stream . id , self . key . stream_id) ; stream . id } # [doc = " Remove the StreamId -> stream state association."] # [doc = ""] # [doc = " This will effectively remove the stream as far as the H2 protocol is"] # [doc = " concerned."] pub fn unlink (& mut self) { let id = self . key . stream_id ; self . store . ids . swap_remove (& id) ; } }
};
}
