// Generated macro for ptrs_to_socket_addr (function)
macro_rules! Depcrate_netptrs_to_socket_addr {
() => {
// Module: crate::net
// Provides: {"ptrs_to_socket_addr"}
// Dependencies: {}
unsafe fn ptrs_to_socket_addr (ptr : * const SOCKADDR , len : i32) -> Option < SocketAddr > { if (len as usize) < mem :: size_of :: < i32 > () { return None ; } match (* ptr) . sa_family as _ { AF_INET if len as usize >= mem :: size_of :: < SOCKADDR_IN > () => { let b = & * (ptr as * const SOCKADDR_IN) ; let ip = ntoh (b . sin_addr . S_un . S_addr) ; let ip = Ipv4Addr :: new ((ip >> 24) as u8 , (ip >> 16) as u8 , (ip >> 8) as u8 , ip as u8 ,) ; Some (SocketAddr :: V4 (SocketAddrV4 :: new (ip , ntoh (b . sin_port)))) } AF_INET6 if len as usize >= mem :: size_of :: < SOCKADDR_IN6 > () => { let b = & * (ptr as * const SOCKADDR_IN6) ; let arr = & b . sin6_addr . u . Byte ; let ip = Ipv6Addr :: new (((arr [0] as u16) << 8) | (arr [1] as u16) , ((arr [2] as u16) << 8) | (arr [3] as u16) , ((arr [4] as u16) << 8) | (arr [5] as u16) , ((arr [6] as u16) << 8) | (arr [7] as u16) , ((arr [8] as u16) << 8) | (arr [9] as u16) , ((arr [10] as u16) << 8) | (arr [11] as u16) , ((arr [12] as u16) << 8) | (arr [13] as u16) , ((arr [14] as u16) << 8) | (arr [15] as u16) ,) ; let addr = SocketAddrV6 :: new (ip , ntoh (b . sin6_port) , ntoh (b . sin6_flowinfo) , ntoh (b . Anonymous . sin6_scope_id) ,) ; Some (SocketAddr :: V6 (addr)) } _ => None , } }
};
}
