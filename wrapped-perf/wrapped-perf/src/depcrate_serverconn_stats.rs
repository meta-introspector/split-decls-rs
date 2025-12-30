// Generated macro for conn_stats (function)
macro_rules! Depcrate_serverconn_stats {
() => {
// Module: crate::server
// Provides: {"conn_stats"}
// Dependencies: {}
async fn conn_stats (connection : quinn :: Connection , opt : Arc < Opt >) -> Result < () > { if opt . common . conn_stats { loop { tokio :: time :: sleep (Duration :: from_secs (2)) . await ; println ! ("{:?}\n" , connection . stats ()) ; } } Ok (()) }
};
}
