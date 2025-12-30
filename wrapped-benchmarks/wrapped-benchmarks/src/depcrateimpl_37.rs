// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl Generate for BlockHeaderInner { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { Self { height : u64 :: generate (rng) , epoch_hash : CryptoHash :: generate (rng) , prev_hash : CryptoHash :: generate (rng) , prev_state_root : MerkleHash :: generate (rng) , tx_root : MerkleHash :: generate (rng) , timestamp : u64 :: generate (rng) , approval_mask : generate_vec_primitives (rng , 2 , 1000) , approval_sigs : generate_vec (rng , 2 , 1000) , total_weight : u64 :: generate (rng) , validator_proposals : generate_vec (rng , 2 , 1000) , } } }
};
}
