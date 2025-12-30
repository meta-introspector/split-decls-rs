// Generated macro for impl_25 (impl)
macro_rules! Depcrate_implsimpl_25 {
() => {
// Module: crate::impls
// Provides: {"impl_25"}
// Dependencies: {}
impl TryFrom < String > for Url { type Error = parse :: Error ; fn try_from (value : String) -> Result < Self , Self :: Error > { Self :: from_bytes (value . as_str () . into ()) } }
};
}
