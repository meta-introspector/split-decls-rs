// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [tokio :: main] async fn main () { tracing_subscriber :: fmt :: init () ; let config = Config :: parse () ; let addr = SocketAddr :: from ((Ipv4Addr :: UNSPECIFIED , config . port)) ; tracing :: info ! ("Listening on {}" , addr) ; axum :: serve (TcpListener :: bind (addr) . await . expect ("bind error") , app () . into_make_service () ,) . await . expect ("server error") ; }
};
}
