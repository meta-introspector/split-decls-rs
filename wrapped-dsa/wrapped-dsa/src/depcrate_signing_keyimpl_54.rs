// Generated macro for impl_54 (impl)
macro_rules! Depcrate_signing_keyimpl_54 {
() => {
// Module: crate::signing_key
// Provides: {"impl_54"}
// Dependencies: {}
impl PrehashSigner < Signature > for SigningKey { # [doc = " Warning: This uses `sha2::Sha256` as the hash function for the digest. If you need to use a different one, use [`SigningKey::sign_prehashed_rfc6979`]."] fn sign_prehash (& self , prehash : & [u8]) -> Result < Signature , signature :: Error > { let k_kinv = crate :: generate :: secret_number_rfc6979 :: < sha2 :: Sha256 > (self , prehash) ? ; self . sign_prehashed (k_kinv , prehash) } }
};
}
