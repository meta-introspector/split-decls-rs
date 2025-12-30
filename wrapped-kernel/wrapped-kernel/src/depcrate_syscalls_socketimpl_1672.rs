// Generated macro for impl_1672 (impl)
macro_rules! Depcrate_syscalls_socketimpl_1672 {
() => {
// Module: crate::syscalls::socket
// Provides: {"impl_1672"}
// Dependencies: {}
impl From < IpAddr > for Af { fn from (value : IpAddr) -> Self { match value { IpAddr :: V4 (_) => Self :: Inet , IpAddr :: V6 (_) => Self :: Inet6 , } } }
};
}
