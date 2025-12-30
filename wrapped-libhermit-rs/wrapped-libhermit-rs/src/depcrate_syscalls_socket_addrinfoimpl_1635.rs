// Generated macro for impl_1635 (impl)
macro_rules! Depcrate_syscalls_socket_addrinfoimpl_1635 {
() => {
// Module: crate::syscalls::socket::addrinfo
// Provides: {"impl_1635"}
// Dependencies: {}
impl addrinfo { fn ai_family (& self) -> Option < Af > { let ai_family = u8 :: try_from (self . ai_family) . ok () ? ; Af :: try_from (ai_family) . ok () } fn ai_socktype (& self) -> Option < (Sock , SockFlags) > { Sock :: from_bits (self . ai_socktype) } fn ai_protocol (& self) -> Option < Ipproto > { let ai_protocol = u8 :: try_from (self . ai_protocol) . ok () ? ; Ipproto :: try_from (ai_protocol) . ok () } fn ai_addr (& self) -> Option < Result < sockaddrRef < '_ > , TryFromPrimitiveError < Af > > > { if self . ai_addr . is_null () { return None ; } let ai_addr = unsafe { & * ptr :: from_ref (& self . ai_addr) . cast () } ; let ret = unsafe { sockaddr :: as_ref (ai_addr) } ; Some (ret) } fn ai_canonname (& self) -> Option < & CStr > { if self . ai_canonname . is_null () { return None ; } let ai_canonname = unsafe { CStr :: from_ptr (self . ai_canonname) } ; Some (ai_canonname) } }
};
}
