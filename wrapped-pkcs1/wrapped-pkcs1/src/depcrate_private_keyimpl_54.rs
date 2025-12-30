// Generated macro for impl_54 (impl)
macro_rules! Depcrate_private_keyimpl_54 {
() => {
// Module: crate::private_key
// Provides: {"impl_54"}
// Dependencies: {}
impl < 'a > From < & RsaPrivateKey < 'a > > for RsaPublicKey < 'a > { fn from (private_key : & RsaPrivateKey < 'a >) -> RsaPublicKey < 'a > { private_key . public_key () } }
};
}
