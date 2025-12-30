// Generated macro for server_listen_and_get_first_connection (function)
macro_rules! Depcrate_connectionserver_listen_and_get_first_connection {
() => {
// Module: crate::connection
// Provides: {"server_listen_and_get_first_connection"}
// Dependencies: {}
# [doc = " Starts listening on given port and return first connection to that port as a stream."] pub fn server_listen_and_get_first_connection (port : & str) -> TcpStream { let listener = TcpListener :: bind ("0.0.0.0:" . to_owned () + port) . unwrap () ; println ! ("Server running, listening for connection on 0.0.0.0:{port}") ; let stream = listener . incoming () . next () . unwrap () . unwrap () ; println ! ("Connection established with {:?}!" , stream . peer_addr () . unwrap ()) ; stream }
};
}
