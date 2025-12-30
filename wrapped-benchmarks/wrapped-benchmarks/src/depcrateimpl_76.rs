// Generated macro for impl_76 (impl)
macro_rules! Depcrateimpl_76 {
() => {
// Module: crate
// Provides: {"impl_76"}
// Dependencies: {}
impl Generate for Account { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { Self { amount : u64 :: generate (rng) , staked : u64 :: generate (rng) , code_hash : CryptoHash :: generate (rng) , storage_usage : u64 :: generate (rng) , storage_paid_at : u64 :: generate (rng) , } } }
};
}
