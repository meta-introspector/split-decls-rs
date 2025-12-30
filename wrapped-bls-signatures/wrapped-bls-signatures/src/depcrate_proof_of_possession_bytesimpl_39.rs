// Generated macro for impl_39 (impl)
macro_rules! Depcrate_proof_of_possession_bytesimpl_39 {
() => {
// Module: crate::proof_of_possession::bytes
// Provides: {"impl_39"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl AsProofOfPossession for [u8 ; BLS_PROOF_OF_POSSESSION_COMPRESSED_SIZE] { fn try_as_affine (& self) -> Result < ProofOfPossession , BlsError > { let compressed = ProofOfPossessionCompressed (* self) ; ProofOfPossession :: try_from (compressed) } }
};
}
