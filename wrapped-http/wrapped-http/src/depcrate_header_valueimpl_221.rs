// Generated macro for impl_221 (impl)
macro_rules! Depcrate_header_valueimpl_221 {
() => {
// Module: crate::header::value
// Provides: {"impl_221"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a String > for HeaderValue { type Error = InvalidHeaderValue ; # [inline] fn try_from (s : & 'a String) -> Result < Self , Self :: Error > { Self :: from_bytes (s . as_bytes ()) } }
};
}
