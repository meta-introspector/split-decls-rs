// Generated macro for impl_24 (impl)
macro_rules! Depcrate_implsimpl_24 {
() => {
// Module: crate::impls
// Provides: {"impl_24"}
// Dependencies: {}
impl TryFrom < & str > for Url { type Error = parse :: Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { Self :: from_bytes (value . into ()) } }
};
}
