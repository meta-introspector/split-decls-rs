// Generated macro for use_3 (pub_use)
macro_rules! Depcrateuse_3 {
() => {
// Module: crate
// Provides: {"use_3"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] pub use crate :: { error :: BlsError , keypair :: Keypair , proof_of_possession :: { AsProofOfPossessionProjective , ProofOfPossessionProjective , VerifiableProofOfPossession , } , pubkey :: { AsPubkeyProjective , PubkeyProjective , VerifiablePubkey } , secret_key :: { SecretKey , BLS_SECRET_KEY_SIZE } , signature :: { AsSignatureProjective , SignatureProjective , VerifiableSignature } , } ;
};
}
