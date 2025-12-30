// Generated macro for impl_160 (impl)
macro_rules! Depcrate_manifestimpl_160 {
() => {
// Module: crate::manifest
// Provides: {"impl_160"}
// Dependencies: {}
impl TryFrom < bool > for WorkspaceValue { type Error = String ; fn try_from (other : bool) -> Result < WorkspaceValue , Self :: Error > { if other { Ok (WorkspaceValue) } else { Err ("`workspace` cannot be false" . to_owned ()) } } }
};
}
