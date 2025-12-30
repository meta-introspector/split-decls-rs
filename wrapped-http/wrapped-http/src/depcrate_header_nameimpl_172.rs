// Generated macro for impl_172 (impl)
macro_rules! Depcrate_header_nameimpl_172 {
() => {
// Module: crate::header::name
// Provides: {"impl_172"}
// Dependencies: {}
impl TryFrom < String > for HeaderName { type Error = InvalidHeaderName ; # [inline] fn try_from (s : String) -> Result < Self , Self :: Error > { Self :: from_bytes (s . as_bytes ()) } }
};
}
