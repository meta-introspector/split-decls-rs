// Generated macro for StoreBufferAlloc (struct)
macro_rules! Depcrate_concurrency_weak_memoryStoreBufferAlloc {
() => {
// Module: crate::concurrency::weak_memory
// Provides: {"StoreBufferAlloc"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct StoreBufferAlloc { # [doc = " Store buffer of each atomic object in this allocation"] store_buffers : RefCell < RangeObjectMap < StoreBuffer > > , }
};
}
