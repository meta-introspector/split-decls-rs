// Generated macro for impl_53 (impl)
macro_rules! Depcrate_private_keyimpl_53 {
() => {
// Module: crate::private_key
// Provides: {"impl_53"}
// Dependencies: {}
impl < 'a > From < RsaPrivateKey < 'a > > for RsaPublicKey < 'a > { fn from (private_key : RsaPrivateKey < 'a >) -> RsaPublicKey < 'a > { private_key . public_key () } }
};
}
