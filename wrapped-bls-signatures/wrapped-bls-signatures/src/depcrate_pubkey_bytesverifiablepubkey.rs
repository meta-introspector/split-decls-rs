// Generated macro for VerifiablePubkey (trait)
macro_rules! Depcrate_pubkey_bytesVerifiablePubkey {
() => {
// Module: crate::pubkey::bytes
// Provides: {"VerifiablePubkey"}
// Dependencies: {}
# [doc = " A trait that provides verification methods to any convertible public key type."] # [cfg (not (target_os = "solana"))] pub trait VerifiablePubkey : AsPubkey { # [doc = " Uses this public key to verify any convertible signature type."] fn verify_signature < S : AsSignature > (& self , signature : & S , message : & [u8] ,) -> Result < bool , BlsError > { let pubkey_affine = self . try_as_affine () ? ; let signature_affine = signature . try_as_affine () ? ; Ok (pubkey_affine . _verify_signature (& signature_affine , message)) } # [doc = " Uses this public key to verify any convertible proof of possession type."] fn verify_proof_of_possession < P : AsProofOfPossession > (& self , proof : & P , payload : Option < & [u8] > ,) -> Result < bool , BlsError > { let pubkey_affine = self . try_as_affine () ? ; let proof_affine = proof . try_as_affine () ? ; Ok (pubkey_affine . _verify_proof_of_possession (& proof_affine , payload)) } }
};
}
