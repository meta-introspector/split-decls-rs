// Generated macro for impl_655 (impl)
macro_rules! Depcrate_ed25519impl_655 {
() => {
// Module: crate::ed25519
// Provides: {"impl_655"}
// Dependencies: {}
impl ParsedVerificationAlgorithm for EdDSAParameters { fn parsed_verify_sig (& self , public_key : & ParsedPublicKey , msg : & [u8] , signature : & [u8] ,) -> Result < () , Unspecified > { public_key . key () . verify (msg , None , No_EVP_PKEY_CTX_consumer , signature) } fn parsed_verify_digest_sig (& self , _public_key : & ParsedPublicKey , _digest : & Digest , _signature : & [u8] ,) -> Result < () , Unspecified > { Err (Unspecified) } }
};
}
