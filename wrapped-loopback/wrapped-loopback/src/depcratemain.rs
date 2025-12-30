// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> io :: Result < () > { let addr = SocketAddr :: from ((Ipv4Addr :: LOCALHOST , 9975)) ; let t = thread :: spawn (move | | { let mut client = TcpStream :: connect (addr) ? ; eprintln ! ("Client successfully connected") ; client . write_all (TO_SEND) ? ; let mut buf = [0u8 ; TO_SEND . len ()] ; client . read_exact (& mut buf) ? ; assert_eq ! (& buf , TO_SEND) ; Ok (()) }) ; let listener = TcpListener :: bind (addr) ? ; eprintln ! ("Listening on {addr}") ; let (socket , socket_addr) = listener . accept () ? ; eprintln ! ("Accepted connection from {socket_addr}") ; handle_client (socket) ? ; t . join () . unwrap () }
};
}
