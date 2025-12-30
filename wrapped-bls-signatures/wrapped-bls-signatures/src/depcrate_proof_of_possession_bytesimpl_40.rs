// Generated macro for impl_40 (impl)
macro_rules! Depcrate_proof_of_possession_bytesimpl_40 {
() => {
// Module: crate::proof_of_possession::bytes
// Provides: {"impl_40"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl AsProofOfPossession for [u8 ; BLS_PROOF_OF_POSSESSION_AFFINE_SIZE] { fn try_as_affine (& self) -> Result < ProofOfPossession , BlsError > { Ok (ProofOfPossession (* self)) } }
};
}
