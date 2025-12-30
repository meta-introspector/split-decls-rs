// Generated macro for copy_unaligned_basic_latin_to_ascii (function)
macro_rules! Depcrate_handlescopy_unaligned_basic_latin_to_ascii {
() => {
// Module: crate::handles
// Provides: {"copy_unaligned_basic_latin_to_ascii"}
// Dependencies: {}
# [cfg (feature = "simd-accel")] # [inline (always)] fn copy_unaligned_basic_latin_to_ascii < E : Endian > (src : UnalignedU16Slice , dst : & mut [u8] ,) -> CopyAsciiResult < usize , (u16 , usize) > { let len = :: core :: cmp :: min (src . len () , dst . len ()) ; let mut offset = 0 ; if SIMD_STRIDE_SIZE <= len { let len_minus_stride = len - SIMD_STRIDE_SIZE ; loop { let mut first = src . simd_at (offset) ; let mut second = src . simd_at (offset + (SIMD_STRIDE_SIZE / 2)) ; if E :: OPPOSITE_ENDIAN { first = simd_byte_swap (first) ; second = simd_byte_swap (second) ; } if ! simd_is_basic_latin (first | second) { break ; } let packed = simd_pack (first , second) ; unsafe { store16_unaligned (dst . as_mut_ptr () . add (offset) , packed) ; } offset += SIMD_STRIDE_SIZE ; if offset > len_minus_stride { break ; } } } copy_unaligned_basic_latin_to_ascii_alu :: < E > (src . tail (offset) , & mut dst [offset ..] , offset) }
};
}
