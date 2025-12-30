// Generated macro for alloc (function)
macro_rules! Depcrate_allocalloc {
() => {
// Module: crate::alloc
// Provides: {"alloc"}
// Dependencies: {}
# [doc = " Allocates memory with the global allocator."] # [doc = ""] # [doc = " This function forwards calls to the [`GlobalAlloc::alloc`] method"] # [doc = " of the allocator registered with the `#[global_allocator]` attribute"] # [doc = " if there is one, or the `std` crate’s default."] # [doc = ""] # [doc = " This function is expected to be deprecated in favor of the `allocate` method"] # [doc = " of the [`Global`] type when it and the [`Allocator`] trait become stable."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See [`GlobalAlloc::alloc`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};"] # [doc = ""] # [doc = " unsafe {"] # [doc = "     let layout = Layout::new::<u16>();"] # [doc = "     let ptr = alloc(layout);"] # [doc = "     if ptr.is_null() {"] # [doc = "         handle_alloc_error(layout);"] # [doc = "     }"] # [doc = ""] # [doc = "     *(ptr as *mut u16) = 42;"] # [doc = "     assert_eq!(*(ptr as *mut u16), 42);"] # [doc = ""] # [doc = "     dealloc(ptr, layout);"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "global_alloc" , since = "1.28.0")] # [must_use = "losing the pointer will leak memory"] # [inline] # [cfg_attr (miri , track_caller)] pub unsafe fn alloc (layout : Layout) -> * mut u8 { unsafe { __rust_no_alloc_shim_is_unstable_v2 () ; __rust_alloc (layout . size () , layout . align ()) } }
};
}
