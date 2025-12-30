// Generated macro for ensure_ipv6 (function)
macro_rules! Depcrate_endpointensure_ipv6 {
() => {
// Module: crate::endpoint
// Provides: {"ensure_ipv6"}
// Dependencies: {}
fn ensure_ipv6 (x : SocketAddr) -> SocketAddrV6 { match x { SocketAddr :: V6 (x) => x , SocketAddr :: V4 (x) => SocketAddrV6 :: new (x . ip () . to_ipv6_mapped () , x . port () , 0 , 0) , } }
};
}
