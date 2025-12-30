// Generated macro for impl_30 (impl)
macro_rules! Depcrate_paramsimpl_30 {
() => {
// Module: crate::params
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for RsaPssParams < 'a > { type Error = Error ; fn try_from (bytes : & 'a [u8]) -> Result < Self > { Ok (Self :: from_der (bytes) ?) } }
};
}
