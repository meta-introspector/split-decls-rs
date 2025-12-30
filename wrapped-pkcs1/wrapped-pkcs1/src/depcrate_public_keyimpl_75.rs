// Generated macro for impl_75 (impl)
macro_rules! Depcrate_public_keyimpl_75 {
() => {
// Module: crate::public_key
// Provides: {"impl_75"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for RsaPublicKey < 'a > { type Error = Error ; fn try_from (bytes : & 'a [u8]) -> Result < Self > { Ok (Self :: from_der (bytes) ?) } }
};
}
