// Generated macro for impl_434 (impl)
macro_rules! Depcrate_uri_pathimpl_434 {
() => {
// Module: crate::uri::path
// Provides: {"impl_434"}
// Dependencies: {}
impl TryFrom < String > for PathAndQuery { type Error = InvalidUri ; # [inline] fn try_from (s : String) -> Result < Self , Self :: Error > { PathAndQuery :: from_shared (s . into ()) } }
};
}
