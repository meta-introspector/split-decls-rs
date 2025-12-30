// Generated macro for impl_216 (impl)
macro_rules! Depcrate_inflate_output_bufferimpl_216 {
() => {
// Module: crate::inflate::output_buffer
// Provides: {"impl_216"}
// Dependencies: {}
impl < 'a > InputWrapper < 'a > { # [inline (always)] pub const fn as_slice (& self) -> & [u8] { self . slice } # [inline (always)] pub const fn from_slice (slice : & 'a [u8]) -> InputWrapper < 'a > { InputWrapper { slice } } # [inline (always)] pub fn advance (& mut self , steps : usize) { self . slice = & self . slice [steps ..] ; } # [inline] pub fn read_byte (& mut self) -> Option < u8 > { self . slice . first () . map (| n | { self . advance (1) ; * n }) } # [inline] # [cfg (target_pointer_width = "64")] pub fn read_u32_le (& mut self) -> u32 { let ret = { let four_bytes : [u8 ; 4] = self . slice [.. 4] . try_into () . unwrap_or_default () ; u32 :: from_le_bytes (four_bytes) } ; self . advance (4) ; ret } # [inline (always)] pub const fn bytes_left (& self) -> usize { self . slice . len () } }
};
}
