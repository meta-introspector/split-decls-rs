// Generated macro for ascii_to_ascii_simd_double_stride (macro)
macro_rules! Depcrate_asciiascii_to_ascii_simd_double_stride {
() => {
// Module: crate::ascii
// Provides: {"ascii_to_ascii_simd_double_stride"}
// Dependencies: {}
# [allow (unused_macros)] macro_rules ! ascii_to_ascii_simd_double_stride { ($ name : ident , $ store : ident) => { # [doc = " Safety: src must be valid for 32 bytes of aligned u8x16 read"] # [doc = " dst must be valid for 32 bytes of unaligned write according to"] # [doc = " the $store fn, which may allow for unaligned writes or require"] # [doc = " alignment to either 16x8 or u8x16."] # [doc = ""] # [doc = " Safety-usable invariant: Returns Some(index) if the element at `index` is invalid ASCII"] # [inline (always)] pub unsafe fn $ name (src : * const u8 , dst : * mut u8) -> Option < usize > { let first = load16_aligned (src) ; let second = load16_aligned (src . add (SIMD_STRIDE_SIZE)) ; $ store (dst , first) ; if unlikely (! simd_is_ascii (first | second)) { let mask_first = mask_ascii (first) ; if mask_first != 0 { return Some (mask_first . trailing_zeros () as usize) ; } $ store (dst . add (SIMD_STRIDE_SIZE) , second) ; let mask_second = mask_ascii (second) ; return Some (SIMD_STRIDE_SIZE + mask_second . trailing_zeros () as usize) ; } $ store (dst . add (SIMD_STRIDE_SIZE) , second) ; None } } ; }
};
}
