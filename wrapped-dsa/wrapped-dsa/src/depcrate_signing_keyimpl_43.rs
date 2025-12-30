// Generated macro for impl_43 (impl)
macro_rules! Depcrate_signing_keyimpl_43 {
() => {
// Module: crate::signing_key
// Provides: {"impl_43"}
// Dependencies: {}
impl Signer < Signature > for SigningKey { fn try_sign (& self , msg : & [u8]) -> Result < Signature , signature :: Error > { self . try_multipart_sign (& [msg]) } }
};
}
