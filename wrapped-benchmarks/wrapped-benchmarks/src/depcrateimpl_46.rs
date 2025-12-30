// Generated macro for impl_46 (impl)
macro_rules! Depcrateimpl_46 {
() => {
// Module: crate
// Provides: {"impl_46"}
// Dependencies: {}
impl Generate for Transaction { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { Self { signer_id : AccountId :: generate (rng) , public_key : PublicKey :: generate (rng) , nonce : u64 :: generate (rng) , receiver_id : AccountId :: generate (rng) , actions : generate_vec (rng , 1 , 10) , } } }
};
}
