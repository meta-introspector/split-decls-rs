// Generated macro for build_bitmap (function)
macro_rules! Depcrate_simd_neonbuild_bitmap {
() => {
// Module: crate::simd::neon
// Provides: {"build_bitmap"}
// Dependencies: {}
const fn build_bitmap () -> ([u8 ; 16] , [u8 ; 16]) { let mut bitmap_0_7 = [0u8 ; 16] ; let mut bitmap_8_15 = [0u8 ; 16] ; let mut i = 0 ; while i < 256 { if bit_set (i as u8) { let (lo , hi) = (i & 0x0F , i >> 4) ; if i < 128 { bitmap_0_7 [lo] |= 1 << hi ; } else { bitmap_8_15 [lo] |= 1 << hi ; } } i += 1 ; } (bitmap_0_7 , bitmap_8_15) }
};
}
