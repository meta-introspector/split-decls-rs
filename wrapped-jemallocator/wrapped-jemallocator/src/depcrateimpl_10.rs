// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
unsafe impl GlobalAlloc for Jemalloc { # [inline] unsafe fn alloc (& self , layout : Layout) -> * mut u8 { assume ! (layout . size () != 0) ; let flags = layout_to_flags (layout . align () , layout . size ()) ; let ptr = if flags == 0 { ffi :: malloc (layout . size ()) } else { ffi :: mallocx (layout . size () , flags) } ; ptr as * mut u8 } # [inline] unsafe fn alloc_zeroed (& self , layout : Layout) -> * mut u8 { assume ! (layout . size () != 0) ; let flags = layout_to_flags (layout . align () , layout . size ()) ; let ptr = if flags == 0 { ffi :: calloc (1 , layout . size ()) } else { ffi :: mallocx (layout . size () , flags | ffi :: MALLOCX_ZERO) } ; ptr as * mut u8 } # [inline] unsafe fn dealloc (& self , ptr : * mut u8 , layout : Layout) { assume ! (! ptr . is_null ()) ; assume ! (layout . size () != 0) ; let flags = layout_to_flags (layout . align () , layout . size ()) ; ffi :: sdallocx (ptr as * mut c_void , layout . size () , flags) } # [inline] unsafe fn realloc (& self , ptr : * mut u8 , layout : Layout , new_size : usize) -> * mut u8 { assume ! (layout . size () != 0) ; assume ! (new_size != 0) ; let flags = layout_to_flags (layout . align () , new_size) ; let ptr = if flags == 0 { ffi :: realloc (ptr as * mut c_void , new_size) } else { ffi :: rallocx (ptr as * mut c_void , new_size , flags) } ; ptr as * mut u8 } }
};
}
