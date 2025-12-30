// Generated macro for impl_70 (impl)
macro_rules! Depcrateimpl_70 {
() => {
// Module: crate
// Provides: {"impl_70"}
// Dependencies: {}
# [cfg (unix)] impl TryFrom < ChildStdout > for OwnedFd { type Error = io :: Error ; fn try_from (value : ChildStdout) -> Result < Self , Self :: Error > { value . 0 . try_into () } }
};
}
