macro_rules! latin1_simd_unalign {
    () => {
        # [allow (unused_macros)] macro_rules ! latin1_simd_unalign { ($ name : ident , $ src_unit : ty , $ dst_unit : ty , $ stride_neither_aligned : ident) => { # [doc = " Safety: src and dst must be valid for unaligned reads/writes of len elements of type src_unit/dst_unit"] # [inline (always)] pub unsafe fn $ name (src : * const $ src_unit , dst : * mut $ dst_unit , len : usize) { let mut offset = 0usize ; if SIMD_STRIDE_SIZE <= len { let len_minus_stride = len - SIMD_STRIDE_SIZE ; loop { $ stride_neither_aligned (src . add (offset) , dst . add (offset)) ; offset += SIMD_STRIDE_SIZE ; if offset > len_minus_stride { break ; } } } while offset < len { let code_unit = * (src . add (offset)) ; * (dst . add (offset)) = code_unit as $ dst_unit ; offset += 1 ; } } } ; }
    };
}

latin1_simd_unalign!()