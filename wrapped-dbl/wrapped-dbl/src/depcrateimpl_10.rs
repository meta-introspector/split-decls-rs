// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl Dbl for Array < u8 , U16 > { # [inline] fn dbl (self) -> Self { let mut val = [u64 :: from_be_bytes (self [.. 8] . try_into () . unwrap ()) , u64 :: from_be_bytes (self [8 ..] . try_into () . unwrap ()) ,] ; let b = val [1] >> 63 ; let a = val [0] >> 63 ; val [0] <<= 1 ; val [0] ^= b ; val [1] <<= 1 ; val [1] ^= a * C128 ; let mut res = Self :: default () ; res [.. 8] . copy_from_slice (& val [0] . to_be_bytes ()) ; res [8 ..] . copy_from_slice (& val [1] . to_be_bytes ()) ; res } # [inline] fn inv_dbl (self) -> Self { let mut val = [u64 :: from_be_bytes (self [.. 8] . try_into () . unwrap ()) , u64 :: from_be_bytes (self [8 ..] . try_into () . unwrap ()) ,] ; let a = (val [0] & 1) << 63 ; let b = val [1] & 1 ; val [0] >>= 1 ; val [1] >>= 1 ; val [1] ^= a ; val [0] ^= b * (1 << 63) ; val [1] ^= b * (C128 >> 1) ; let mut res = Self :: default () ; res [.. 8] . copy_from_slice (& val [0] . to_be_bytes ()) ; res [8 ..] . copy_from_slice (& val [1] . to_be_bytes ()) ; res } }
};
}
