// Generated macro for impl_351 (impl)
macro_rules! Depcrate_statusimpl_351 {
() => {
// Module: crate::status
// Provides: {"impl_351"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for StatusCode { type Error = InvalidStatusCode ; # [inline] fn try_from (t : & 'a [u8]) -> Result < Self , Self :: Error > { StatusCode :: from_bytes (t) } }
};
}
