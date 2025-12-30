// Generated macro for impl_222 (impl)
macro_rules! Depcrate_header_valueimpl_222 {
() => {
// Module: crate::header::value
// Provides: {"impl_222"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for HeaderValue { type Error = InvalidHeaderValue ; # [inline] fn try_from (t : & 'a [u8]) -> Result < Self , Self :: Error > { HeaderValue :: from_bytes (t) } }
};
}
