// Generated macro for handle_push (function)
macro_rules! Depcratehandle_push {
() => {
// Module: crate
// Provides: {"handle_push"}
// Dependencies: {}
fn handle_push (socket : TcpStream , work : & Path , config : Config) { let mut reader = BufReader :: new (socket) ; let dst = recv (& work , & mut reader) ; print_verbose (& format ! ("push {:#?}" , dst) , config) ; let mut socket = reader . into_inner () ; t ! (socket . write_all (b"ack ")) ; }
};
}
