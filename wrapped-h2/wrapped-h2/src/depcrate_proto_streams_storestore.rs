// Generated macro for Store (struct)
macro_rules! Depcrate_proto_streams_storeStore {
() => {
// Module: crate::proto::streams::store
// Provides: {"Store"}
// Dependencies: {}
# [doc = " Storage for streams"] # [derive (Debug)] pub (super) struct Store { slab : slab :: Slab < Stream > , ids : IndexMap < StreamId , SlabIndex > , }
};
}
