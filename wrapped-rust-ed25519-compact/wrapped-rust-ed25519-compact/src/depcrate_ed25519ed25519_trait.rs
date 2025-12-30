// Generated macro for ed25519_trait (module)
macro_rules! Depcrate_ed25519ed25519_trait {
() => {
// Module: crate::ed25519
// Provides: {"ed25519_trait"}
// Dependencies: {}
# [cfg (feature = "traits")] mod ed25519_trait { use :: ed25519 :: signature as ed25519_trait ; use super :: { PublicKey , SecretKey , Signature } ; impl ed25519_trait :: SignatureEncoding for Signature { type Repr = Signature ; } impl ed25519_trait :: Signer < Signature > for SecretKey { fn try_sign (& self , message : & [u8]) -> Result < Signature , ed25519_trait :: Error > { Ok (self . sign (message , None)) } } impl ed25519_trait :: Verifier < Signature > for PublicKey { fn verify (& self , message : & [u8] , signature : & Signature ,) -> Result < () , ed25519_trait :: Error > { # [cfg (feature = "std")] { self . verify (message , signature) . map_err (ed25519_trait :: Error :: from_source) } # [cfg (not (feature = "std"))] { self . verify (message , signature) . map_err (| _ | ed25519_trait :: Error :: new ()) } } } }
};
}
