// Generated macro for handle_connection (function)
macro_rules! Depcratehandle_connection {
() => {
// Module: crate
// Provides: {"handle_connection"}
// Dependencies: {}
fn handle_connection (mut stream : TcpStream) { let buf_reader = BufReader :: new (& stream) ; let http_request : Vec < _ > = buf_reader . lines () . map (| result | result . unwrap ()) . take_while (| line | ! line . is_empty ()) . collect () ; println ! ("Request: {http_request:#?}") ; }
};
}
