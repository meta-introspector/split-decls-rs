// Generated macro for std_addr_from_c (function)
macro_rules! Depcrate_ffistd_addr_from_c {
() => {
// Module: crate::ffi
// Provides: {"std_addr_from_c"}
// Dependencies: {}
fn std_addr_from_c (addr : & sockaddr , addr_len : socklen_t) -> SocketAddr { match addr . sa_family as _ { AF_INET => { assert ! (addr_len as usize == size_of ::< sockaddr_in > ()) ; let in4 = unsafe { * (addr as * const _ as * const sockaddr_in) } ; # [cfg (not (windows))] let ip_addr = Ipv4Addr :: from (u32 :: from_be (in4 . sin_addr . s_addr)) ; # [cfg (windows)] let ip_addr = { let ip_bytes = unsafe { in4 . sin_addr . S_un . S_un_b } ; Ipv4Addr :: from ([ip_bytes . s_b1 , ip_bytes . s_b2 , ip_bytes . s_b3 , ip_bytes . s_b4 ,]) } ; let port = u16 :: from_be (in4 . sin_port) ; let out = SocketAddrV4 :: new (ip_addr , port) ; out . into () } , AF_INET6 => { assert ! (addr_len as usize == size_of ::< sockaddr_in6 > ()) ; let in6 = unsafe { * (addr as * const _ as * const sockaddr_in6) } ; let ip_addr = Ipv6Addr :: from (# [cfg (not (windows))] in6 . sin6_addr . s6_addr , # [cfg (windows)] unsafe { in6 . sin6_addr . u . Byte } ,) ; let port = u16 :: from_be (in6 . sin6_port) ; # [cfg (not (windows))] let scope_id = in6 . sin6_scope_id ; # [cfg (windows)] let scope_id = unsafe { in6 . Anonymous . sin6_scope_id } ; let out = SocketAddrV6 :: new (ip_addr , port , in6 . sin6_flowinfo , scope_id) ; out . into () } , _ => unimplemented ! ("unsupported address type") , } }
};
}
