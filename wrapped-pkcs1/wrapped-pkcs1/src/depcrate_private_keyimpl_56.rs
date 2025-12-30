// Generated macro for impl_56 (impl)
macro_rules! Depcrate_private_keyimpl_56 {
() => {
// Module: crate::private_key
// Provides: {"impl_56"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a OctetStringRef > for RsaPrivateKey < 'a > { type Error = Error ; fn try_from (bytes : & 'a OctetStringRef) -> Result < Self > { Ok (Self :: from_der (bytes . as_bytes ()) ?) } }
};
}
