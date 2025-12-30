// Generated macro for impl_57 (impl)
macro_rules! Depcrate_signing_keyimpl_57 {
() => {
// Module: crate::signing_key
// Provides: {"impl_57"}
// Dependencies: {}
impl < D > RandomizedDigestSigner < D , Signature > for SigningKey where D : EagerHash + Update , { fn try_sign_digest_with_rng < R : TryCryptoRng + ? Sized , F : Fn (& mut D) -> Result < () , signature :: Error > , > (& self , rng : & mut R , f : F ,) -> Result < Signature , signature :: Error > { let ks = crate :: generate :: secret_number (rng , self . verifying_key () . components ()) ? . ok_or_else (signature :: Error :: new) ? ; let mut digest = D :: new () ; f (& mut digest) ? ; let hash = digest . finalize () ; self . sign_prehashed (ks , & hash) } }
};
}
