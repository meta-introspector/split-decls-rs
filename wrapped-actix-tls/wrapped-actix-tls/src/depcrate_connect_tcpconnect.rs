// Generated macro for connect (function)
macro_rules! Depcrate_connect_tcpconnect {
() => {
// Module: crate::connect::tcp
// Provides: {"connect"}
// Dependencies: {}
async fn connect (addr : SocketAddr , local_addr : Option < IpAddr >) -> io :: Result < TcpStream > { match local_addr { Some (ip_addr) => { let socket = match ip_addr { IpAddr :: V4 (ip_addr) => { let socket = TcpSocket :: new_v4 () ? ; let addr = SocketAddr :: V4 (SocketAddrV4 :: new (ip_addr , 0)) ; socket . bind (addr) ? ; socket } IpAddr :: V6 (ip_addr) => { let socket = TcpSocket :: new_v6 () ? ; let addr = SocketAddr :: V6 (SocketAddrV6 :: new (ip_addr , 0 , 0 , 0)) ; socket . bind (addr) ? ; socket } } ; socket . connect (addr) . await } None => TcpStream :: connect (addr) . await , } }
};
}
