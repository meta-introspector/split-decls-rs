// Generated macro for impl_1644 (impl)
macro_rules! Depcrate_syscalls_socket_addrinfoimpl_1644 {
() => {
// Module: crate::syscalls::socket::addrinfo
// Provides: {"impl_1644"}
// Dependencies: {}
impl < 'a > Iterator for addrinfoIter < 'a > { type Item = & 'a addrinfo ; fn next (& mut self) -> Option < Self :: Item > { let next = self . 0 ? ; self . 0 = next . ai_next . as_deref () ; Some (next) } }
};
}
