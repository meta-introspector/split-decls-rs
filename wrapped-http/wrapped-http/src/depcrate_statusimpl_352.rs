// Generated macro for impl_352 (impl)
macro_rules! Depcrate_statusimpl_352 {
() => {
// Module: crate::status
// Provides: {"impl_352"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a str > for StatusCode { type Error = InvalidStatusCode ; # [inline] fn try_from (t : & 'a str) -> Result < Self , Self :: Error > { t . parse () } }
};
}
