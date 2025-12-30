// Generated macro for impl_257 (impl)
macro_rules! Depcrate_rngimpl_257 {
() => {
// Module: crate::rng
// Provides: {"impl_257"}
// Dependencies: {}
impl Fill for u8 { fn fill_slice < R : Rng + ? Sized > (this : & mut [Self] , rng : & mut R) { rng . fill_bytes (this) } }
};
}
