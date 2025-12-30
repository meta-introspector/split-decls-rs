// Generated macro for impl_100 (impl)
macro_rules! Depcrate_socketimpl_100 {
() => {
// Module: crate::socket
// Provides: {"impl_100"}
// Dependencies: {}
impl From < StdTcpListener > for MioListener { fn from (lst : StdTcpListener) -> Self { MioListener :: Tcp (MioTcpListener :: from_std (lst)) } }
};
}
