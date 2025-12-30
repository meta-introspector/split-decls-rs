// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl Dbl for Array < u8 , U8 > { # [inline] fn dbl (self) -> Self { let mut val = u64 :: from_be_bytes (self . into ()) ; let a = val >> 63 ; val <<= 1 ; val ^= a * C64 ; val . to_be_bytes () . into () } # [inline] fn inv_dbl (self) -> Self { let mut val = u64 :: from_be_bytes (self . into ()) ; let a = val & 1 ; val >>= 1 ; val ^= a * ((1 << 63) ^ (C64 >> 1)) ; val . to_be_bytes () . into () } }
};
}
