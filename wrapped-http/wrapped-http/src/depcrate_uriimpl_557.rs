// Generated macro for impl_557 (impl)
macro_rules! Depcrate_uriimpl_557 {
() => {
// Module: crate::uri
// Provides: {"impl_557"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a Uri > for Uri { type Error = crate :: Error ; # [inline] fn try_from (src : & 'a Uri) -> Result < Self , Self :: Error > { Ok (src . clone ()) } }
};
}
