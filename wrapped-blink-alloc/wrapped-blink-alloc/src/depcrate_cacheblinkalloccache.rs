// Generated macro for BlinkAllocCache (struct)
macro_rules! Depcrate_cacheBlinkAllocCache {
() => {
// Module: crate::cache
// Provides: {"BlinkAllocCache"}
// Dependencies: {}
# [doc = " Multi-thread cache for [`BlinkAlloc`] instances."] # [doc = " Stores pushed [`BlinkAlloc`] instances and returns them on pop."] # [doc = " Blink-allocators are kept warm in the cache."] # [doc = ""] # [doc = " This type is internally synchronized with hybrid"] # [doc = " blocking + wait-free algorithm."] pub struct BlinkAllocCache < A : Allocator = Global > { inner : RwLock < Inner < A > > , }
};
}
