// Generated macro for impl_70 (impl)
macro_rules! Depcrate_verifying_keyimpl_70 {
() => {
// Module: crate::verifying_key
// Provides: {"impl_70"}
// Dependencies: {}
impl MultipartVerifier < Signature > for VerifyingKey { fn multipart_verify (& self , msg : & [& [u8]] , signature : & Signature ,) -> Result < () , signature :: Error > { self . verify_digest (| digest : & mut sha2 :: Sha256 | { msg . iter () . for_each (| slice | digest . update (slice)) ; Ok (()) } , signature ,) } }
};
}
