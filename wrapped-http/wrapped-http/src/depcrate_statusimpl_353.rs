// Generated macro for impl_353 (impl)
macro_rules! Depcrate_statusimpl_353 {
() => {
// Module: crate::status
// Provides: {"impl_353"}
// Dependencies: {}
impl TryFrom < u16 > for StatusCode { type Error = InvalidStatusCode ; # [inline] fn try_from (t : u16) -> Result < Self , Self :: Error > { StatusCode :: from_u16 (t) } }
};
}
