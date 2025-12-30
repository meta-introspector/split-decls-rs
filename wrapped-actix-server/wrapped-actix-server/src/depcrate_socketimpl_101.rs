// Generated macro for impl_101 (impl)
macro_rules! Depcrate_socketimpl_101 {
() => {
// Module: crate::socket
// Provides: {"impl_101"}
// Dependencies: {}
# [cfg (unix)] impl From < StdUnixListener > for MioListener { fn from (lst : StdUnixListener) -> Self { MioListener :: Uds (MioUnixListener :: from_std (lst)) } }
};
}
