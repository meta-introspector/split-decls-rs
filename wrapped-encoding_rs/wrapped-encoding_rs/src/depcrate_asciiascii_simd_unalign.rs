// Generated macro for ascii_simd_unalign (macro)
macro_rules! Depcrate_asciiascii_simd_unalign {
() => {
// Module: crate::ascii
// Provides: {"ascii_simd_unalign"}
// Dependencies: {}
# [allow (unused_macros)] macro_rules ! ascii_simd_unalign { ($ name : ident , $ src_unit : ty , $ dst_unit : ty , $ stride_neither_aligned : ident) => { # [doc = " Safety: src and dst must be valid for reads/writes of len elements of type src_unit/dst_unit"] # [doc = ""] # [doc = " Safety-usable invariant: will return Some() when it encounters non-ASCII, with the first element in the Some being"] # [doc = " guaranteed to be non-ASCII (> 127), and the second being the offset where it is found"] # [inline (always)] pub unsafe fn $ name (src : * const $ src_unit , dst : * mut $ dst_unit , len : usize ,) -> Option < ($ src_unit , usize) > { let mut offset = 0usize ; if SIMD_STRIDE_SIZE <= len { let len_minus_stride = len - SIMD_STRIDE_SIZE ; loop { if !$ stride_neither_aligned (src . add (offset) , dst . add (offset)) { break ; } offset += SIMD_STRIDE_SIZE ; if offset > len_minus_stride { break ; } } } while offset < len { let code_unit = * (src . add (offset)) ; if code_unit > 127 { return Some ((code_unit , offset)) ; } * (dst . add (offset)) = code_unit as $ dst_unit ; offset += 1 ; } None } } ; }
};
}
