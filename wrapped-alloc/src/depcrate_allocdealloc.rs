// Generated macro for dealloc (function)
macro_rules! Depcrate_allocdealloc {
() => {
// Module: crate::alloc
// Provides: {"dealloc"}
// Dependencies: {}
# [doc = " Deallocates memory with the global allocator."] # [doc = ""] # [doc = " This function forwards calls to the [`GlobalAlloc::dealloc`] method"] # [doc = " of the allocator registered with the `#[global_allocator]` attribute"] # [doc = " if there is one, or the `std` crate’s default."] # [doc = ""] # [doc = " This function is expected to be deprecated in favor of the `deallocate` method"] # [doc = " of the [`Global`] type when it and the [`Allocator`] trait become stable."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See [`GlobalAlloc::dealloc`]."] # [stable (feature = "global_alloc" , since = "1.28.0")] # [inline] # [cfg_attr (miri , track_caller)] pub unsafe fn dealloc (ptr : * mut u8 , layout : Layout) { unsafe { __rust_dealloc (ptr , layout . size () , layout . align ()) } }
};
}
