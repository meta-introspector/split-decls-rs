// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl Generate for Signature { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { let mut res = [0u8 ; 32] ; rng . fill_bytes (& mut res) ; Signature (res) } }
};
}
