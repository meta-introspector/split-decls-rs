// Generated macro for impl_38 (impl)
macro_rules! Depcrate_uniximpl_38 {
() => {
// Module: crate::unix
// Provides: {"impl_38"}
// Dependencies: {}
# [cfg (unix)] impl TryFrom < OwnedFd > for UnixStream { type Error = io :: Error ; fn try_from (value : OwnedFd) -> Result < Self , Self :: Error > { Self :: try_from (std :: os :: unix :: net :: UnixStream :: from (value)) } }
};
}
