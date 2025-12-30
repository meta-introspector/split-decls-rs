// Generated macro for AsProofOfPossession (trait)
macro_rules! Depcrate_proof_of_possession_bytesAsProofOfPossession {
() => {
// Module: crate::proof_of_possession::bytes
// Provides: {"AsProofOfPossession"}
// Dependencies: {}
# [doc = " A trait for types that can be converted into a `ProofOfPossession` (affine)."] # [cfg (not (target_os = "solana"))] pub trait AsProofOfPossession { # [doc = " Attempt to convert the type into a `ProofOfPossession`."] fn try_as_affine (& self) -> Result < ProofOfPossession , BlsError > ; }
};
}
