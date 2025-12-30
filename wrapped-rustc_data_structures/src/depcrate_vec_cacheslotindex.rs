// Generated macro for SlotIndex (struct)
macro_rules! Depcrate_vec_cacheSlotIndex {
() => {
// Module: crate::vec_cache
// Provides: {"SlotIndex"}
// Dependencies: {}
# [doc = " This uniquely identifies a single `Slot<V>` entry in the buckets map, and provides accessors for"] # [doc = " either getting the value or putting a value."] # [derive (Copy , Clone , Debug)] struct SlotIndex { bucket_idx : usize , entries : usize , index_in_bucket : usize , }
};
}
