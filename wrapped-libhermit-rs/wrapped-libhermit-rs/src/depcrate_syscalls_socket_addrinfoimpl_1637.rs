// Generated macro for impl_1637 (impl)
macro_rules! Depcrate_syscalls_socket_addrinfoimpl_1637 {
() => {
// Module: crate::syscalls::socket::addrinfo
// Provides: {"impl_1637"}
// Dependencies: {}
impl Drop for addrinfo { fn drop (& mut self) { if ! self . ai_addr . is_null () { let ai_addr = unsafe { sockaddr :: as_box (self . ai_addr) . unwrap () } ; drop (ai_addr) ; } if ! self . ai_canonname . is_null () { let ai_canonname = unsafe { CString :: from_raw (self . ai_canonname) } ; drop (ai_canonname) ; } } }
};
}
