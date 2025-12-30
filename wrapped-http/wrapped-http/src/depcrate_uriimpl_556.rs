// Generated macro for impl_556 (impl)
macro_rules! Depcrate_uriimpl_556 {
() => {
// Module: crate::uri
// Provides: {"impl_556"}
// Dependencies: {}
impl TryFrom < Parts > for Uri { type Error = InvalidUriParts ; # [inline] fn try_from (src : Parts) -> Result < Self , Self :: Error > { Uri :: from_parts (src) } }
};
}
