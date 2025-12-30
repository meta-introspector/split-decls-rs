// Generated macro for impl_80 (impl)
macro_rules! Depcrate_verifying_keyimpl_80 {
() => {
// Module: crate::verifying_key
// Provides: {"impl_80"}
// Dependencies: {}
impl PrehashVerifier < Signature > for VerifyingKey { fn verify_prehash (& self , prehash : & [u8] , signature : & Signature ,) -> Result < () , signature :: Error > { if let Some (true) = self . verify_prehashed (prehash , signature) { Ok (()) } else { Err (signature :: Error :: new ()) } } }
};
}
