// Generated macro for impl_78 (impl)
macro_rules! Depcrate_verifying_keyimpl_78 {
() => {
// Module: crate::verifying_key
// Provides: {"impl_78"}
// Dependencies: {}
impl Verifier < Signature > for VerifyingKey { fn verify (& self , msg : & [u8] , signature : & Signature) -> Result < () , signature :: Error > { self . multipart_verify (& [msg] , signature) } }
};
}
