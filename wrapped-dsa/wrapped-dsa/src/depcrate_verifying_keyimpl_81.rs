// Generated macro for impl_81 (impl)
macro_rules! Depcrate_verifying_keyimpl_81 {
() => {
// Module: crate::verifying_key
// Provides: {"impl_81"}
// Dependencies: {}
impl < D > DigestVerifier < D , Signature > for VerifyingKey where D : EagerHash + Update , { fn verify_digest < F : Fn (& mut D) -> Result < () , signature :: Error > > (& self , f : F , signature : & Signature ,) -> Result < () , signature :: Error > { let mut digest = D :: new () ; f (& mut digest) ? ; let hash = digest . finalize () ; let is_valid = self . verify_prehashed (& hash , signature) . ok_or_else (signature :: Error :: new) ? ; if ! is_valid { return Err (signature :: Error :: new ()) ; } Ok (()) } }
};
}
