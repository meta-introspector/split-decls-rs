// Generated macro for impl_22 (impl)
macro_rules! Depcrate_uniximpl_22 {
() => {
// Module: crate::unix
// Provides: {"impl_22"}
// Dependencies: {}
# [cfg (unix)] impl TryFrom < OwnedFd > for UnixListener { type Error = io :: Error ; fn try_from (value : OwnedFd) -> Result < Self , Self :: Error > { Self :: try_from (std :: os :: unix :: net :: UnixListener :: from (value)) } }
};
}
