// Generated macro for impl_58 (impl)
macro_rules! Depcrate_private_keyimpl_58 {
() => {
// Module: crate::private_key
// Provides: {"impl_58"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl TryFrom < RsaPrivateKey < '_ > > for SecretDocument { type Error = Error ; fn try_from (private_key : RsaPrivateKey < '_ >) -> Result < SecretDocument > { SecretDocument :: try_from (& private_key) } }
};
}
