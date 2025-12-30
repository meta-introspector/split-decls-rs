// Generated macro for impl_303 (impl)
macro_rules! Depcrate_rngimpl_303 {
() => {
// Module: crate::rng
// Provides: {"impl_303"}
// Dependencies: {}
impl Fill for u8 { fn fill_slice < R : Rng + ? Sized > (this : & mut [Self] , rng : & mut R) { rng . fill_bytes (this) } }
};
}
