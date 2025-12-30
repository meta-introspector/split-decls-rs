// Generated macro for impl_69 (impl)
macro_rules! Depcrate_verifying_keyimpl_69 {
() => {
// Module: crate::verifying_key
// Provides: {"impl_69"}
// Dependencies: {}
impl Verifier < Signature > for VerifyingKey { fn verify (& self , msg : & [u8] , signature : & Signature) -> Result < () , signature :: Error > { self . multipart_verify (& [msg] , signature) } }
};
}
