// Generated macro for bind_local_address (function)
macro_rules! Depcrate_client_legacy_connect_httpbind_local_address {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"bind_local_address"}
// Dependencies: {}
fn bind_local_address (socket : & socket2 :: Socket , dst_addr : & SocketAddr , local_addr_ipv4 : & Option < Ipv4Addr > , local_addr_ipv6 : & Option < Ipv6Addr > ,) -> io :: Result < () > { match (* dst_addr , local_addr_ipv4 , local_addr_ipv6) { (SocketAddr :: V4 (_) , Some (addr) , _) => { socket . bind (& SocketAddr :: new ((* addr) . into () , 0) . into ()) ? ; } (SocketAddr :: V6 (_) , _ , Some (addr)) => { socket . bind (& SocketAddr :: new ((* addr) . into () , 0) . into ()) ? ; } _ => { if cfg ! (windows) { let any : SocketAddr = match * dst_addr { SocketAddr :: V4 (_) => ([0 , 0 , 0 , 0] , 0) . into () , SocketAddr :: V6 (_) => ([0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , 0) . into () , } ; socket . bind (& any . into ()) ? ; } } } Ok (()) }
};
}
