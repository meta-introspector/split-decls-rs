// Generated macro for impl_223 (impl)
macro_rules! Depcrate_header_valueimpl_223 {
() => {
// Module: crate::header::value
// Provides: {"impl_223"}
// Dependencies: {}
impl TryFrom < String > for HeaderValue { type Error = InvalidHeaderValue ; # [inline] fn try_from (t : String) -> Result < Self , Self :: Error > { HeaderValue :: from_shared (t . into ()) } }
};
}
