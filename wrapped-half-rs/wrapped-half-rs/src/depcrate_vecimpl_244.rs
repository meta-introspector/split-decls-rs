// Generated macro for impl_244 (impl)
macro_rules! Depcrate_vecimpl_244 {
() => {
// Module: crate::vec
// Provides: {"impl_244"}
// Dependencies: {}
impl HalfFloatVecExt for Vec < bf16 > { # [inline] fn reinterpret_into (mut self) -> Vec < u16 > { let length = self . len () ; let capacity = self . capacity () ; let pointer = self . as_mut_ptr () as * mut u16 ; mem :: forget (self) ; unsafe { Vec :: from_raw_parts (pointer , length , capacity) } } # [allow (clippy :: uninit_vec)] fn from_f32_slice (slice : & [f32]) -> Self { let mut vec = vec ! [bf16 :: from_bits (0) ; slice . len ()] ; vec . convert_from_f32_slice (slice) ; vec } # [allow (clippy :: uninit_vec)] fn from_f64_slice (slice : & [f64]) -> Self { let mut vec = vec ! [bf16 :: from_bits (0) ; slice . len ()] ; vec . convert_from_f64_slice (slice) ; vec } }
};
}
