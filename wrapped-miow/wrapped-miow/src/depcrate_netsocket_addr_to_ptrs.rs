// Generated macro for socket_addr_to_ptrs (function)
macro_rules! Depcrate_netsocket_addr_to_ptrs {
() => {
// Module: crate::net
// Provides: {"socket_addr_to_ptrs"}
// Dependencies: {}
fn socket_addr_to_ptrs (addr : & SocketAddr) -> (SocketAddrCRepr , i32) { match * addr { SocketAddr :: V4 (ref a) => { let sin_addr = IN_ADDR { S_un : IN_ADDR_0 { S_addr : u32 :: from_ne_bytes (a . ip () . octets ()) , } , } ; let sockaddr_in = SOCKADDR_IN { sin_family : AF_INET as _ , sin_port : a . port () . to_be () , sin_addr , sin_zero : [0 ; 8] , } ; let sockaddr = SocketAddrCRepr { v4 : sockaddr_in } ; (sockaddr , mem :: size_of :: < SOCKADDR_IN > () as i32) } SocketAddr :: V6 (ref a) => { let sockaddr_in6 = SOCKADDR_IN6 { sin6_family : AF_INET6 as _ , sin6_port : a . port () . to_be () , sin6_addr : IN6_ADDR { u : IN6_ADDR_0 { Byte : a . ip () . octets () , } , } , sin6_flowinfo : a . flowinfo () , Anonymous : SOCKADDR_IN6_0 { sin6_scope_id : a . scope_id () , } , } ; let sockaddr = SocketAddrCRepr { v6 : sockaddr_in6 } ; (sockaddr , mem :: size_of :: < SOCKADDR_IN6 > () as i32) } } }
};
}
