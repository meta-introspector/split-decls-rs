// Generated macro for impl_29 (impl)
macro_rules! Depcrate_implsimpl_29 {
() => {
// Module: crate::impls
// Provides: {"impl_29"}
// Dependencies: {}
impl TryFrom < & BStr > for Url { type Error = parse :: Error ; fn try_from (value : & BStr) -> Result < Self , Self :: Error > { Self :: from_bytes (value) } }
};
}
