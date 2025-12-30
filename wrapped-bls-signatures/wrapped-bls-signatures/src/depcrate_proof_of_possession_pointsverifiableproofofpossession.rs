// Generated macro for VerifiableProofOfPossession (trait)
macro_rules! Depcrate_proof_of_possession_pointsVerifiableProofOfPossession {
() => {
// Module: crate::proof_of_possession::points
// Provides: {"VerifiableProofOfPossession"}
// Dependencies: {}
# [doc = " A trait that provides verification methods to any convertible proof of possession type."] # [cfg (not (target_os = "solana"))] pub trait VerifiableProofOfPossession : AsProofOfPossessionProjective { # [doc = " Verifies the proof of possession against any convertible public key type."] fn verify < P : VerifiablePubkey > (& self , pubkey : & P , payload : Option < & [u8] > ,) -> Result < bool , BlsError > { let proof_projective = self . try_as_projective () ? ; pubkey . verify_proof_of_possession (& proof_projective , payload) } }
};
}
