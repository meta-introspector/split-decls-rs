// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl Generate for PublicKey { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { let mut res = [0u8 ; 32] ; rng . fill_bytes (& mut res) ; PublicKey (res) } }
};
}
