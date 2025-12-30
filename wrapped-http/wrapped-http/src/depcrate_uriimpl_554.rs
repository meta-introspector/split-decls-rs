// Generated macro for impl_554 (impl)
macro_rules! Depcrate_uriimpl_554 {
() => {
// Module: crate::uri
// Provides: {"impl_554"}
// Dependencies: {}
impl TryFrom < String > for Uri { type Error = InvalidUri ; # [inline] fn try_from (t : String) -> Result < Self , Self :: Error > { Uri :: from_shared (Bytes :: from (t)) } }
};
}
