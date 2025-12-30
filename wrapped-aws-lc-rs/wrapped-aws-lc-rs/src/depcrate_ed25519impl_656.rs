// Generated macro for impl_656 (impl)
macro_rules! Depcrate_ed25519impl_656 {
() => {
// Module: crate::ed25519
// Provides: {"impl_656"}
// Dependencies: {}
impl VerificationAlgorithm for EdDSAParameters { # [inline] # [cfg (feature = "ring-sig-verify")] fn verify (& self , public_key : Input < '_ > , msg : Input < '_ > , signature : Input < '_ > ,) -> Result < () , Unspecified > { self . verify_sig (public_key . as_slice_less_safe () , msg . as_slice_less_safe () , signature . as_slice_less_safe () ,) } # [doc = " Verify `signature` for `msg` using `public_key`."] # [doc = ""] # [doc = " # Errors"] # [doc = "  Returns `Unspecified` if the `msg` cannot be verified using `public_key`."] fn verify_sig (& self , public_key : & [u8] , msg : & [u8] , signature : & [u8] ,) -> Result < () , Unspecified > { let evp_pkey = parse_ed25519_public_key (public_key) ? ; evp_pkey . verify (msg , None , No_EVP_PKEY_CTX_consumer , signature) } # [doc = " DO NOT USE. This function is required by `VerificationAlgorithm` but cannot be used w/ Ed25519."] # [doc = ""] # [doc = " # Errors"] # [doc = " Always returns `Unspecified`."] fn verify_digest_sig (& self , _public_key : & [u8] , _digest : & digest :: Digest , _signature : & [u8] ,) -> Result < () , Unspecified > { Err (Unspecified) } }
};
}
