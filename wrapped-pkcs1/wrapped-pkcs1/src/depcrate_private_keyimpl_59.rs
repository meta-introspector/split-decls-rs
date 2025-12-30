// Generated macro for impl_59 (impl)
macro_rules! Depcrate_private_keyimpl_59 {
() => {
// Module: crate::private_key
// Provides: {"impl_59"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl TryFrom < & RsaPrivateKey < '_ > > for SecretDocument { type Error = Error ; fn try_from (private_key : & RsaPrivateKey < '_ >) -> Result < SecretDocument > { Ok (Self :: encode_msg (private_key) ?) } }
};
}
