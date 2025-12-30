// Generated macro for impl_170 (impl)
macro_rules! Depcrate_header_nameimpl_170 {
() => {
// Module: crate::header::name
// Provides: {"impl_170"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a String > for HeaderName { type Error = InvalidHeaderName ; # [inline] fn try_from (s : & 'a String) -> Result < Self , Self :: Error > { Self :: from_bytes (s . as_bytes ()) } }
};
}
