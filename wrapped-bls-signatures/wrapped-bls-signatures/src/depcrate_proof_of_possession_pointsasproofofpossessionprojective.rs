// Generated macro for AsProofOfPossessionProjective (trait)
macro_rules! Depcrate_proof_of_possession_pointsAsProofOfPossessionProjective {
() => {
// Module: crate::proof_of_possession::points
// Provides: {"AsProofOfPossessionProjective"}
// Dependencies: {}
# [doc = " A trait for types that can be converted into a `ProofOfPossessionProjective`."] # [cfg (not (target_os = "solana"))] pub trait AsProofOfPossessionProjective { # [doc = " Attempt to convert the type into a `ProofOfPossessionProjective`."] fn try_as_projective (& self) -> Result < ProofOfPossessionProjective , BlsError > ; }
};
}
