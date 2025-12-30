// Generated macro for impl_351 (impl)
macro_rules! Depcrate_genericimpl_351 {
() => {
// Module: crate::generic
// Provides: {"impl_351"}
// Dependencies: {}
impl StoreBytes for u32x4_generic { # [inline (always)] unsafe fn unsafe_read_le (input : & [u8]) -> Self { let x = u32x4_generic :: read_from_bytes (input) . unwrap () ; dmap (x , | x | x . to_le ()) } # [inline (always)] unsafe fn unsafe_read_be (input : & [u8]) -> Self { let x = u32x4_generic :: read_from_bytes (input) . unwrap () ; dmap (x , | x | x . to_be ()) } # [inline (always)] fn write_le (self , out : & mut [u8]) { let x = dmap (self , | x | x . to_le ()) ; x . write_to (out) . unwrap () ; } # [inline (always)] fn write_be (self , out : & mut [u8]) { let x = dmap (self , | x | x . to_be ()) ; x . write_to (out) . unwrap () ; } }
};
}
