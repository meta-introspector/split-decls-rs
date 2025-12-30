// Generated macro for impl_431 (impl)
macro_rules! Depcrate_uri_pathimpl_431 {
() => {
// Module: crate::uri::path
// Provides: {"impl_431"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for PathAndQuery { type Error = InvalidUri ; # [inline] fn try_from (s : & 'a [u8]) -> Result < Self , Self :: Error > { PathAndQuery :: from_shared (Bytes :: copy_from_slice (s)) } }
};
}
