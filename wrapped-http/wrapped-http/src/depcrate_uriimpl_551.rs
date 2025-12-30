// Generated macro for impl_551 (impl)
macro_rules! Depcrate_uriimpl_551 {
() => {
// Module: crate::uri
// Provides: {"impl_551"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for Uri { type Error = InvalidUri ; # [inline] fn try_from (t : & 'a [u8]) -> Result < Self , Self :: Error > { Uri :: from_shared (Bytes :: copy_from_slice (t)) } }
};
}
