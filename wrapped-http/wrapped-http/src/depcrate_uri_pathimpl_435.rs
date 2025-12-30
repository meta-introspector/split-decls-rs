// Generated macro for impl_435 (impl)
macro_rules! Depcrate_uri_pathimpl_435 {
() => {
// Module: crate::uri::path
// Provides: {"impl_435"}
// Dependencies: {}
impl TryFrom < & String > for PathAndQuery { type Error = InvalidUri ; # [inline] fn try_from (s : & String) -> Result < Self , Self :: Error > { TryFrom :: try_from (s . as_bytes ()) } }
};
}
