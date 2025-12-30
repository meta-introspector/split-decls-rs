// Generated macro for impl_171 (impl)
macro_rules! Depcrate_header_nameimpl_171 {
() => {
// Module: crate::header::name
// Provides: {"impl_171"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for HeaderName { type Error = InvalidHeaderName ; # [inline] fn try_from (s : & 'a [u8]) -> Result < Self , Self :: Error > { Self :: from_bytes (s) } }
};
}
