// Generated macro for impl_79 (impl)
macro_rules! Depcrate_verifying_keyimpl_79 {
() => {
// Module: crate::verifying_key
// Provides: {"impl_79"}
// Dependencies: {}
impl MultipartVerifier < Signature > for VerifyingKey { fn multipart_verify (& self , msg : & [& [u8]] , signature : & Signature ,) -> Result < () , signature :: Error > { self . verify_digest (| digest : & mut sha2 :: Sha256 | { msg . iter () . for_each (| slice | digest . update (slice)) ; Ok (()) } , signature ,) } }
};
}
