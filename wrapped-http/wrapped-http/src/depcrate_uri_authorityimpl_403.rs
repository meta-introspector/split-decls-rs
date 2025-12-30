// Generated macro for impl_403 (impl)
macro_rules! Depcrate_uri_authorityimpl_403 {
() => {
// Module: crate::uri::authority
// Provides: {"impl_403"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a str > for Authority { type Error = InvalidUri ; # [inline] fn try_from (s : & 'a str) -> Result < Self , Self :: Error > { TryFrom :: try_from (s . as_bytes ()) } }
};
}
