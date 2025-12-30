// Generated macro for impl_402 (impl)
macro_rules! Depcrate_uri_authorityimpl_402 {
() => {
// Module: crate::uri::authority
// Provides: {"impl_402"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for Authority { type Error = InvalidUri ; # [inline] fn try_from (s : & 'a [u8]) -> Result < Self , Self :: Error > { create_authority (s , Bytes :: copy_from_slice) } }
};
}
