// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl Generate for MerkleHash { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { let mut res = [0u8 ; 32] ; rng . fill_bytes (& mut res) ; MerkleHash (res) } }
};
}
