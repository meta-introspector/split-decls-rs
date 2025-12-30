// Generated macro for impl_169 (impl)
macro_rules! Depcrate_header_nameimpl_169 {
() => {
// Module: crate::header::name
// Provides: {"impl_169"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a str > for HeaderName { type Error = InvalidHeaderName ; # [inline] fn try_from (s : & 'a str) -> Result < Self , Self :: Error > { Self :: from_bytes (s . as_bytes ()) } }
};
}
