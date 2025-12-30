// Generated macro for impl_483 (impl)
macro_rules! Depcrate_uri_schemeimpl_483 {
() => {
// Module: crate::uri::scheme
// Provides: {"impl_483"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a str > for Scheme { type Error = InvalidUri ; # [inline] fn try_from (s : & 'a str) -> Result < Self , Self :: Error > { TryFrom :: try_from (s . as_bytes ()) } }
};
}
