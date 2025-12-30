// Generated macro for impl_38 (impl)
macro_rules! Depcrate_paramsimpl_38 {
() => {
// Module: crate::params
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for RsaOaepParams < 'a > { type Error = Error ; fn try_from (bytes : & 'a [u8]) -> Result < Self > { Ok (Self :: from_der (bytes) ?) } }
};
}
