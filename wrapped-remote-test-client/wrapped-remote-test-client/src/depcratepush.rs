// Generated macro for push (function)
macro_rules! Depcratepush {
() => {
// Module: crate
// Provides: {"push"}
// Dependencies: {}
fn push (path : & Path) { let device_address = env :: var (REMOTE_ADDR_ENV) . unwrap_or (DEFAULT_ADDR . to_string ()) ; let client = t ! (TcpStream :: connect (device_address)) ; let mut client = BufWriter :: new (client) ; t ! (client . write_all (b"push")) ; send (path , & mut client) ; t ! (client . flush ()) ; let mut client = client . into_inner () . unwrap () ; let mut buf = [0 ; 4] ; t ! (client . read_exact (& mut buf)) ; assert_eq ! (& buf , b"ack ") ; println ! ("done pushing {:?}" , path) ; }
};
}
