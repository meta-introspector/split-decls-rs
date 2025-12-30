// Generated macro for impl_76 (impl)
macro_rules! Depcrateimpl_76 {
() => {
// Module: crate
// Provides: {"impl_76"}
// Dependencies: {}
# [cfg (unix)] impl TryFrom < ChildStderr > for OwnedFd { type Error = io :: Error ; fn try_from (value : ChildStderr) -> Result < Self , Self :: Error > { value . 0 . try_into () } }
};
}
