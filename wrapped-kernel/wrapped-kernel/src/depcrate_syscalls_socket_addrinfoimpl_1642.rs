// Generated macro for impl_1642 (impl)
macro_rules! Depcrate_syscalls_socket_addrinfoimpl_1642 {
() => {
// Module: crate::syscalls::socket::addrinfo
// Provides: {"impl_1642"}
// Dependencies: {}
impl FromIterator < addrinfo > for addrinfoList { fn from_iter < T : IntoIterator < Item = addrinfo > > (iter : T) -> Self { let mut res = Self :: default () ; res . extend (iter) ; res } }
};
}
