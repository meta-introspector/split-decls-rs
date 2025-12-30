// Generated macro for impl_1639 (impl)
macro_rules! Depcrate_syscalls_socket_addrinfoimpl_1639 {
() => {
// Module: crate::syscalls::socket::addrinfo
// Provides: {"impl_1639"}
// Dependencies: {}
impl addrinfoList { fn is_empty (& self) -> bool { self . 0 . is_none () } fn iter (& self) -> addrinfoIter < '_ > { addrinfoIter (self . 0 . as_deref ()) } }
};
}
