// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
# [doc = " Integer arithmetic in this global allocator implementation is safe when"] # [doc = " operating on the prescribed `HEAP_START_ADDRESS` and `HEAP_LENGTH`. Any"] # [doc = " other use may overflow and is thus unsupported and at one's own risk."] # [allow (clippy :: arithmetic_side_effects)] unsafe impl std :: alloc :: GlobalAlloc for BumpAllocator { # [inline] unsafe fn alloc (& self , layout : Layout) -> * mut u8 { let pos_ptr = self . start as * mut usize ; let mut pos = * pos_ptr ; if pos == 0 { pos = self . start + self . len ; } pos = pos . saturating_sub (layout . size ()) ; pos &= ! (layout . align () . wrapping_sub (1)) ; if pos < self . start + size_of :: < * mut u8 > () { return null_mut () ; } * pos_ptr = pos ; pos as * mut u8 } # [inline] unsafe fn dealloc (& self , _ : * mut u8 , _ : Layout) { } }
};
}
