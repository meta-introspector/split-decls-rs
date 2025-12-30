// Generated macro for impl_286 (impl)
macro_rules! Depcrate_methodimpl_286 {
() => {
// Module: crate::method
// Provides: {"impl_286"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a str > for Method { type Error = InvalidMethod ; # [inline] fn try_from (t : & 'a str) -> Result < Self , Self :: Error > { TryFrom :: try_from (t . as_bytes ()) } }
};
}
