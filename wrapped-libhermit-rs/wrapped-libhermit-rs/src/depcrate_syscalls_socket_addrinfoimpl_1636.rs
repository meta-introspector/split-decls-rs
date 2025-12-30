// Generated macro for impl_1636 (impl)
macro_rules! Depcrate_syscalls_socket_addrinfoimpl_1636 {
() => {
// Module: crate::syscalls::socket::addrinfo
// Provides: {"impl_1636"}
// Dependencies: {}
impl fmt :: Debug for addrinfo { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("addrinfo") . field ("ai_flags" , & self . ai_flags) . field ("ai_family" , & self . ai_family ()) . field ("ai_socktype" , & self . ai_socktype ()) . field ("ai_protocol" , & self . ai_protocol ()) . field ("ai_addrlen" , & self . ai_addrlen) . field ("ai_addr" , & self . ai_addr ()) . field ("ai_canonname" , & self . ai_canonname ()) . finish () } }
};
}
