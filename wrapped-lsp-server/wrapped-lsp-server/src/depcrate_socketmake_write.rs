// Generated macro for make_write (function)
macro_rules! Depcrate_socketmake_write {
() => {
// Module: crate::socket
// Provides: {"make_write"}
// Dependencies: {}
fn make_write (mut stream : TcpStream ,) -> (Sender < Message > , thread :: JoinHandle < io :: Result < () > > , Receiver < Message >) { let (writer_sender , writer_receiver) = bounded :: < Message > (0) ; let (drop_sender , drop_receiver) = bounded :: < Message > (0) ; let writer = thread :: spawn (move | | { writer_receiver . into_iter () . try_for_each (| it | { let result = it . write (& mut stream) ; let _ = drop_sender . send (it) ; result }) . unwrap () ; Ok (()) }) ; (writer_sender , writer , drop_receiver) }
};
}
