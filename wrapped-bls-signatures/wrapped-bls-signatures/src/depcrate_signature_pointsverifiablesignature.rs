// Generated macro for VerifiableSignature (trait)
macro_rules! Depcrate_signature_pointsVerifiableSignature {
() => {
// Module: crate::signature::points
// Provides: {"VerifiableSignature"}
// Dependencies: {}
# [doc = " A trait that provides verification methods to any convertible signature type."] # [cfg (not (target_os = "solana"))] pub trait VerifiableSignature : AsSignatureProjective { # [doc = " Verify the signature against any convertible public key type and a message."] fn verify < P : VerifiablePubkey > (& self , pubkey : & P , message : & [u8]) -> Result < bool , BlsError > { let signature_projective = self . try_as_projective () ? ; pubkey . verify_signature (& signature_projective , message) } }
};
}
