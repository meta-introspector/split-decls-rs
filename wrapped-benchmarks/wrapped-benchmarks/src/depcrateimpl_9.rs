// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl Generate for CryptoHash { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { let mut res = [0u8 ; 32] ; rng . fill_bytes (& mut res) ; CryptoHash (res) } }
};
}
