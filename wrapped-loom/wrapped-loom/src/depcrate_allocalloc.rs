// Generated macro for alloc (function)
macro_rules! Depcrate_allocalloc {
() => {
// Module: crate::alloc
// Provides: {"alloc"}
// Dependencies: {}
# [doc = " Allocate memory with the global allocator."] # [doc = ""] # [doc = " This is equivalent to the standard library's [`std::alloc::alloc`], but with"] # [doc = " the addition of leak tracking for allocated objects. Loom's leak tracking"] # [doc = " will not function for allocations not performed via this method."] # [doc = ""] # [doc = " This function forwards calls to the [`GlobalAlloc::alloc`] method"] # [doc = " of the allocator registered with the `#[global_allocator]` attribute"] # [doc = " if there is one, or the `std` crate’s default."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See [`GlobalAlloc::alloc`]."] # [doc = ""] # [doc = " [`GlobalAlloc::alloc`]: std::alloc::GlobalAlloc::alloc"] # [track_caller] pub unsafe fn alloc (layout : Layout) -> * mut u8 { let ptr = std :: alloc :: alloc (layout) ; rt :: alloc (ptr , location ! ()) ; ptr }
};
}
