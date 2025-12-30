// Generated macro for impl_246 (impl)
macro_rules! Depcrate_storageimpl_246 {
() => {
// Module: crate::storage
// Provides: {"impl_246"}
// Dependencies: {}
impl SealedStorage for ViewStorage { type Buffer < T > = [T] ; fn len < T > (this : * const Self :: Buffer < T >) -> usize { this . len () } fn as_ptr < T > (this : * mut Self :: Buffer < T >) -> * mut T { this . cast () } # [cfg (any (feature = "portable-atomic" , all (feature = "mpmc_large" , target_has_atomic = "ptr") , all (not (feature = "mpmc_large") , target_has_atomic = "8")))] fn as_mpmc_view < T > (this : & mpmc :: QueueInner < T , Self >) -> & mpmc :: QueueView < T > where Self : Storage + Sized , { this } # [cfg (any (feature = "portable-atomic" , all (feature = "mpmc_large" , target_has_atomic = "ptr") , all (not (feature = "mpmc_large") , target_has_atomic = "8")))] fn as_mpmc_mut_view < T > (this : & mut mpmc :: QueueInner < T , Self >) -> & mut mpmc :: QueueView < T > where Self : Storage + Sized , { this } # [cfg (any (feature = "portable-atomic" , target_has_atomic = "ptr" , has_atomic_load_store))] # [doc = " Convert a `Queue` to a `QueueView`"] fn as_queue_view < T > (this : & spsc :: QueueInner < T , Self >) -> & spsc :: QueueView < T > where Self : Storage + Sized , { this } # [cfg (any (feature = "portable-atomic" , target_has_atomic = "ptr" , has_atomic_load_store))] # [doc = " Convert a `Queue` to a `QueueView`"] fn as_mut_queue_view < T > (this : & mut spsc :: QueueInner < T , Self >) -> & mut spsc :: QueueView < T > where Self : Storage + Sized , { this } }
};
}
