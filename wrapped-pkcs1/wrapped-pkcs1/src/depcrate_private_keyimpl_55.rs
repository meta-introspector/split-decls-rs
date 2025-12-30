// Generated macro for impl_55 (impl)
macro_rules! Depcrate_private_keyimpl_55 {
() => {
// Module: crate::private_key
// Provides: {"impl_55"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for RsaPrivateKey < 'a > { type Error = Error ; fn try_from (bytes : & 'a [u8]) -> Result < Self > { Ok (Self :: from_der (bytes) ?) } }
};
}
