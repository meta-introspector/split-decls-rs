// Generated macro for impl_220 (impl)
macro_rules! Depcrate_header_valueimpl_220 {
() => {
// Module: crate::header::value
// Provides: {"impl_220"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a str > for HeaderValue { type Error = InvalidHeaderValue ; # [inline] fn try_from (t : & 'a str) -> Result < Self , Self :: Error > { t . parse () } }
};
}
