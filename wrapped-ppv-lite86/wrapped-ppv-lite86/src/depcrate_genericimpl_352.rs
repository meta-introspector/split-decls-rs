// Generated macro for impl_352 (impl)
macro_rules! Depcrate_genericimpl_352 {
() => {
// Module: crate::generic
// Provides: {"impl_352"}
// Dependencies: {}
impl StoreBytes for u64x2_generic { # [inline (always)] unsafe fn unsafe_read_le (input : & [u8]) -> Self { let x = u64x2_generic :: read_from_bytes (input) . unwrap () ; qmap (x , | x | x . to_le ()) } # [inline (always)] unsafe fn unsafe_read_be (input : & [u8]) -> Self { let x = u64x2_generic :: read_from_bytes (input) . unwrap () ; qmap (x , | x | x . to_be ()) } # [inline (always)] fn write_le (self , out : & mut [u8]) { let x = qmap (self , | x | x . to_le ()) ; x . write_to (out) . unwrap () ; } # [inline (always)] fn write_be (self , out : & mut [u8]) { let x = qmap (self , | x | x . to_be ()) ; x . write_to (out) . unwrap () ; } }
};
}
