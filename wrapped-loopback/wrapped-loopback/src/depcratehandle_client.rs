// Generated macro for handle_client (function)
macro_rules! Depcratehandle_client {
() => {
// Module: crate
// Provides: {"handle_client"}
// Dependencies: {}
fn handle_client (mut stream : TcpStream) -> io :: Result < () > { let mut buf = [0u8 ; TO_SEND . len ()] ; stream . read_exact (& mut buf) ? ; assert_eq ! (& buf , TO_SEND) ; stream . write_all (TO_SEND) }
};
}
