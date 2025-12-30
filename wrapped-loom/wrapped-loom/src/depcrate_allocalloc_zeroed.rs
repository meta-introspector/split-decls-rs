// Generated macro for alloc_zeroed (function)
macro_rules! Depcrate_allocalloc_zeroed {
() => {
// Module: crate::alloc
// Provides: {"alloc_zeroed"}
// Dependencies: {}
# [doc = " Allocate zero-initialized memory with the global allocator."] # [doc = ""] # [doc = " This is equivalent to the standard library's [`std::alloc::alloc_zeroed`],"] # [doc = " but with the addition of leak tracking for allocated objects. Loom's leak"] # [doc = " tracking will not function for allocations not performed via this method."] # [doc = ""] # [doc = " This function forwards calls to the [`GlobalAlloc::alloc_zeroed`] method"] # [doc = " of the allocator registered with the `#[global_allocator]` attribute"] # [doc = " if there is one, or the `std` crate’s default."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See [`GlobalAlloc::alloc_zeroed`]."] # [doc = ""] # [doc = " [`GlobalAlloc::alloc_zeroed`]: std::alloc::GlobalAlloc::alloc_zeroed"] # [track_caller] pub unsafe fn alloc_zeroed (layout : Layout) -> * mut u8 { let ptr = std :: alloc :: alloc_zeroed (layout) ; rt :: alloc (ptr , location ! ()) ; ptr }
};
}
