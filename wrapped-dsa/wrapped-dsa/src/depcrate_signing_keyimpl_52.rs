// Generated macro for impl_52 (impl)
macro_rules! Depcrate_signing_keyimpl_52 {
() => {
// Module: crate::signing_key
// Provides: {"impl_52"}
// Dependencies: {}
impl Signer < Signature > for SigningKey { fn try_sign (& self , msg : & [u8]) -> Result < Signature , signature :: Error > { self . try_multipart_sign (& [msg]) } }
};
}
