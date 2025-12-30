// Generated macro for impl_64 (impl)
macro_rules! Depcrateimpl_64 {
() => {
// Module: crate
// Provides: {"impl_64"}
// Dependencies: {}
# [cfg (unix)] impl TryFrom < ChildStdin > for OwnedFd { type Error = io :: Error ; fn try_from (value : ChildStdin) -> Result < Self , Self :: Error > { value . 0 . try_into () } }
};
}
