// Generated macro for realloc (function)
macro_rules! Depcrate_allocrealloc {
() => {
// Module: crate::alloc
// Provides: {"realloc"}
// Dependencies: {}
# [doc = " Reallocates memory with the global allocator."] # [doc = ""] # [doc = " This function forwards calls to the [`GlobalAlloc::realloc`] method"] # [doc = " of the allocator registered with the `#[global_allocator]` attribute"] # [doc = " if there is one, or the `std` crate’s default."] # [doc = ""] # [doc = " This function is expected to be deprecated in favor of the `grow` and `shrink` methods"] # [doc = " of the [`Global`] type when it and the [`Allocator`] trait become stable."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See [`GlobalAlloc::realloc`]."] # [stable (feature = "global_alloc" , since = "1.28.0")] # [must_use = "losing the pointer will leak memory"] # [inline] # [cfg_attr (miri , track_caller)] pub unsafe fn realloc (ptr : * mut u8 , layout : Layout , new_size : usize) -> * mut u8 { unsafe { __rust_realloc (ptr , layout . size () , layout . align () , new_size) } }
};
}
