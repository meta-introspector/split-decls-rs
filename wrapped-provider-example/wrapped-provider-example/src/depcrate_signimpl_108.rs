// Generated macro for impl_108 (impl)
macro_rules! Depcrate_signimpl_108 {
() => {
// Module: crate::sign
// Provides: {"impl_108"}
// Dependencies: {}
impl Signer for EcdsaSigningKeyP256 { fn sign (self : Box < Self > , message : & [u8]) -> Result < Vec < u8 > , rustls :: Error > { self . key . try_sign_with_rng (& mut rand_core :: OsRng , message) . map_err (| _ | rustls :: Error :: General ("signing failed" . into ())) . map (| sig : p256 :: ecdsa :: DerSignature | sig . to_vec ()) } fn scheme (& self) -> SignatureScheme { self . scheme } }
};
}
