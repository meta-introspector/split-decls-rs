// Generated macro for impl_50 (impl)
macro_rules! Depcrate_rngimpl_50 {
() => {
// Module: crate::rng
// Provides: {"impl_50"}
// Dependencies: {}
impl From < [u8 ; Self :: BYTES] > for U32x2 { # [inline] fn from (value : [u8 ; Self :: BYTES]) -> Self { let mut result = Self (Default :: default ()) ; for (cur , chunk) in result . 0 . iter_mut () . zip (value . chunks_exact (size_of :: < u32 > ())) { * cur = u32 :: from_le_bytes (chunk . try_into () . unwrap ()) ; } result } }
};
}
