// Generated macro for to_ipv4_mapped (function)
macro_rules! Depcrateto_ipv4_mapped {
() => {
// Module: crate
// Provides: {"to_ipv4_mapped"}
// Dependencies: {}
# [doc = " Copied from unstable `std::net::ip::Ipv6Addr::to_ipv4_mapped`."] # [must_use] const fn to_ipv4_mapped (addr : & Ipv6Addr) -> Option < Ipv4Addr > { match addr . octets () { [0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0xff , 0xff , a , b , c , d] => Some (Ipv4Addr :: new (a , b , c , d)) , _ => None , } }
};
}
