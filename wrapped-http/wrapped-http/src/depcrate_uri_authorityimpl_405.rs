// Generated macro for impl_405 (impl)
macro_rules! Depcrate_uri_authorityimpl_405 {
() => {
// Module: crate::uri::authority
// Provides: {"impl_405"}
// Dependencies: {}
impl TryFrom < String > for Authority { type Error = InvalidUri ; # [inline] fn try_from (t : String) -> Result < Self , Self :: Error > { Authority :: from_shared (t . into ()) } }
};
}
