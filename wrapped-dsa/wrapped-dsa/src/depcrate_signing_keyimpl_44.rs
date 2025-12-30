// Generated macro for impl_44 (impl)
macro_rules! Depcrate_signing_keyimpl_44 {
() => {
// Module: crate::signing_key
// Provides: {"impl_44"}
// Dependencies: {}
impl MultipartSigner < Signature > for SigningKey { fn try_multipart_sign (& self , msg : & [& [u8]]) -> Result < Signature , signature :: Error > { self . try_sign_digest (| digest : & mut sha2 :: Sha256 | { msg . iter () . for_each (| slice | digest . update (slice)) ; Ok (()) }) } }
};
}
