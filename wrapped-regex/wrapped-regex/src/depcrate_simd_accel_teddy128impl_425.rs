// Generated macro for impl_425 (impl)
macro_rules! Depcrate_simd_accel_teddy128impl_425 {
() => {
// Module: crate::simd_accel::teddy128
// Provides: {"impl_425"}
// Dependencies: {}
impl Mask { # [doc = " Create a new mask with no members."] fn new () -> Mask { Mask { lo : u8x16 :: splat (0) , hi : u8x16 :: splat (0) , } } # [doc = " Adds the given byte to the given bucket."] fn add (& mut self , bucket : u8 , byte : u8) { let byte_lo = (byte & 0xF) as u32 ; let byte_hi = (byte >> 4) as u32 ; let lo = self . lo . extract (byte_lo) ; self . lo = self . lo . replace (byte_lo , ((1 << bucket) as u8) | lo) ; let hi = self . hi . extract (byte_hi) ; self . hi = self . hi . replace (byte_hi , ((1 << bucket) as u8) | hi) ; } }
};
}
