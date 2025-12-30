// Generated macro for impl_51 (impl)
macro_rules! Depcrate_rngimpl_51 {
() => {
// Module: crate::rng
// Provides: {"impl_51"}
// Dependencies: {}
impl From < u64 > for U32x2 { # [inline] fn from (value : u64) -> Self { let result : [u8 ; Self :: BYTES] = value . to_le_bytes () [.. Self :: BYTES] . try_into () . unwrap () ; result . into () } }
};
}
