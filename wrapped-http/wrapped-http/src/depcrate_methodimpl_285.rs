// Generated macro for impl_285 (impl)
macro_rules! Depcrate_methodimpl_285 {
() => {
// Module: crate::method
// Provides: {"impl_285"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for Method { type Error = InvalidMethod ; # [inline] fn try_from (t : & 'a [u8]) -> Result < Self , Self :: Error > { Method :: from_bytes (t) } }
};
}
