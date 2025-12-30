// Generated macro for impl_56 (impl)
macro_rules! Depcrate_signing_keyimpl_56 {
() => {
// Module: crate::signing_key
// Provides: {"impl_56"}
// Dependencies: {}
impl < D > DigestSigner < D , Signature > for SigningKey where D : EagerHash + Update , { fn try_sign_digest < F : Fn (& mut D) -> Result < () , signature :: Error > > (& self , f : F ,) -> Result < Signature , signature :: Error > { let mut digest = D :: new () ; f (& mut digest) ? ; let hash = digest . finalize () ; let ks = crate :: generate :: secret_number_rfc6979 :: < D > (self , & hash) ? ; self . sign_prehashed (ks , & hash) } }
};
}
