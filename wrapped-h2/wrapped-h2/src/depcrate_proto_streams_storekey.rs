// Generated macro for Key (struct)
macro_rules! Depcrate_proto_streams_storeKey {
() => {
// Module: crate::proto::streams::store
// Provides: {"Key"}
// Dependencies: {}
# [doc = " References an entry in the store."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub (crate) struct Key { index : SlabIndex , # [doc = " Keep the stream ID in the key as an ABA guard, since slab indices"] # [doc = " could be re-used with a new stream."] stream_id : StreamId , }
};
}
