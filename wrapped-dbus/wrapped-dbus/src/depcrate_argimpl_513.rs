// Generated macro for impl_513 (impl)
macro_rules! Depcrate_argimpl_513 {
() => {
// Module: crate::arg
// Provides: {"impl_513"}
// Dependencies: {}
# [cfg (not (feature = "stdfd"))] impl Clone for OwnedFd { # [cfg (unix)] fn clone (& self) -> OwnedFd { self . try_clone () . unwrap () } # [cfg (windows)] fn clone (& self) -> OwnedFd { OwnedFd { } } }
};
}
