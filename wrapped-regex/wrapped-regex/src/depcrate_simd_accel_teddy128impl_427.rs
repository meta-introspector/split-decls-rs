// Generated macro for impl_427 (impl)
macro_rules! Depcrate_simd_accel_teddy128impl_427 {
() => {
// Module: crate::simd_accel::teddy128
// Provides: {"impl_427"}
// Dependencies: {}
impl UnsafeLoad for u8x16 { type Elem = u8 ; unsafe fn load_unchecked (slice : & [u8] , offset : usize) -> u8x16 { let mut x = u8x16 :: splat (0) ; ptr :: copy_nonoverlapping (slice . get_unchecked (offset) , & mut x as * mut u8x16 as * mut u8 , 16) ; x } }
};
}
