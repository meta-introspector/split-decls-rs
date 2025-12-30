// Generated macro for macro_472 (macro)
macro_rules! Depcrate_memmacro_472 {
() => {
// Module: crate::mem
// Provides: {"macro_472"}
// Dependencies: {}
cfg_if ! { if # [cfg (all (feature = "simd-accel" , any (target_feature = "sse2" , all (target_endian = "little" , target_arch = "aarch64") , all (target_endian = "little" , target_feature = "neon"))))] { # [inline (always)] fn is_utf16_bidi_impl (buffer : & [u16]) -> bool { let mut offset = 0usize ; let len = buffer . len () ; if len >= SIMD_STRIDE_SIZE / 2 { let src = buffer . as_ptr () ; let mut until_alignment = ((SIMD_ALIGNMENT - ((src as usize) & SIMD_ALIGNMENT_MASK)) & SIMD_ALIGNMENT_MASK) / 2 ; if until_alignment + (SIMD_STRIDE_SIZE / 2) <= len { while until_alignment != 0 { if is_utf16_code_unit_bidi (buffer [offset]) { return true ; } offset += 1 ; until_alignment -= 1 ; } let len_minus_stride = len - (SIMD_STRIDE_SIZE / 2) ; loop { if is_u16x8_bidi (unsafe { * (src . add (offset) as * const u16x8) }) { return true ; } offset += SIMD_STRIDE_SIZE / 2 ; if offset > len_minus_stride { break ; } } } } for & u in & buffer [offset ..] { if is_utf16_code_unit_bidi (u) { return true ; } } false } } else { # [inline (always)] fn is_utf16_bidi_impl (buffer : & [u16]) -> bool { for & u in buffer { if is_utf16_code_unit_bidi (u) { return true ; } } false } } }
};
}
