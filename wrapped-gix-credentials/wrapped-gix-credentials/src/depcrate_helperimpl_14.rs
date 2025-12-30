// Generated macro for impl_14 (impl)
macro_rules! Depcrate_helperimpl_14 {
() => {
// Module: crate::helper
// Provides: {"impl_14"}
// Dependencies: {}
impl TryFrom < & NextAction > for Context { type Error = protocol :: context :: decode :: Error ; fn try_from (value : & NextAction) -> std :: result :: Result < Self , Self :: Error > { Context :: from_bytes (value . previous_output . as_ref ()) } }
};
}
