// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let server = tiny_http :: Server :: http ("0.0.0.0:9975") . unwrap () ; eprintln ! ("Now listening on port 9975") ; for request in server . incoming_requests () { handle_request (request) ; if cfg ! (feature = "ci") { break ; } } }
};
}
