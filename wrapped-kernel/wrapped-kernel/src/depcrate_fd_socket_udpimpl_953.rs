// Generated macro for impl_953 (impl)
macro_rules! Depcrate_fd_socket_udpimpl_953 {
() => {
// Module: crate::fd::socket::udp
// Provides: {"impl_953"}
// Dependencies: {}
impl Socket { pub fn new (handle : Handle , domain : Af) -> Self { let local_endpoint = if domain == Af :: Inet { IpEndpoint :: new (Ipv4Address :: UNSPECIFIED . into () , 0) } else if domain == Af :: Inet6 { IpEndpoint :: new (Ipv6Address :: UNSPECIFIED . into () , 0) } else { panic ! ("Unsupported domain for TCP socket: {domain:?}") ; } ; Self { handle , nonblocking : false , local_endpoint , remote_endpoint : None , } } fn with < R > (& self , f : impl FnOnce (& mut udp :: Socket < '_ >) -> R) -> R { let mut guard = NIC . lock () ; let nic = guard . as_nic_mut () . unwrap () ; f (nic . get_mut_socket :: < udp :: Socket < '_ > > (self . handle)) } async fn close (& self) -> io :: Result < () > { self . with (| socket | socket . close ()) ; Ok (()) } async fn write_with_meta (& self , buffer : & [u8] , meta : & UdpMetadata) -> io :: Result < usize > { future :: poll_fn (| cx | { self . with (| socket | { if socket . is_open () { if socket . can_send () { Poll :: Ready (socket . send_slice (buffer , * meta) . map (| () | buffer . len ()) . map_err (| _ | Errno :: Io) ,) } else { socket . register_recv_waker (cx . waker ()) ; Poll :: Pending } } else { Poll :: Ready (Err (Errno :: Io)) } }) }) . await } }
};
}
