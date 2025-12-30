// Generated macro for impl_46 (impl)
macro_rules! Depcrate_signing_keyimpl_46 {
() => {
// Module: crate::signing_key
// Provides: {"impl_46"}
// Dependencies: {}
impl RandomizedPrehashSigner < Signature > for SigningKey { fn sign_prehash_with_rng < R : TryCryptoRng + ? Sized > (& self , rng : & mut R , prehash : & [u8] ,) -> Result < Signature , signature :: Error > { let components = self . verifying_key . components () ; if let Some (k_kinv) = crate :: generate :: secret_number (rng , components) ? { self . sign_prehashed (k_kinv , prehash) } else { Err (signature :: Error :: new ()) } } }
};
}
