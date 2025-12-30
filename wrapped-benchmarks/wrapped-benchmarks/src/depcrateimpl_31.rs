// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl Generate for ValidatorStake { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { Self { account_id : AccountId :: generate (rng) , public_key : PublicKey :: generate (rng) , amount : u64 :: generate (rng) , } } }
};
}
