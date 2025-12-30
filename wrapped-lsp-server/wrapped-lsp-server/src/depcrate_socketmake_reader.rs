// Generated macro for make_reader (function)
macro_rules! Depcrate_socketmake_reader {
() => {
// Module: crate::socket
// Provides: {"make_reader"}
// Dependencies: {}
fn make_reader (stream : TcpStream) -> (Receiver < Message > , thread :: JoinHandle < io :: Result < () > >) { let (reader_sender , reader_receiver) = bounded :: < Message > (0) ; let reader = thread :: spawn (move | | { let mut buf_read = BufReader :: new (stream) ; while let Some (msg) = Message :: read (& mut buf_read) . unwrap () { let is_exit = matches ! (& msg , Message :: Notification (n) if n . is_exit ()) ; reader_sender . send (msg) . unwrap () ; if is_exit { break ; } } Ok (()) }) ; (reader_receiver , reader) }
};
}
