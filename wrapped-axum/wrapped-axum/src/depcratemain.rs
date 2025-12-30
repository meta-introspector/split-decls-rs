// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [tokio :: main (flavor = "current_thread")] async fn main () -> io :: Result < () > { println ! ("Testing axum...") ; let app = Router :: new () . route ("/" , get (root)) ; let listener = net :: TcpListener :: bind ("0.0.0.0:9975") . await ? ; axum :: serve (listener , app) . await ? ; Ok (()) }
};
}
