// Generated macro for impl_432 (impl)
macro_rules! Depcrate_uri_pathimpl_432 {
() => {
// Module: crate::uri::path
// Provides: {"impl_432"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a str > for PathAndQuery { type Error = InvalidUri ; # [inline] fn try_from (s : & 'a str) -> Result < Self , Self :: Error > { TryFrom :: try_from (s . as_bytes ()) } }
};
}
