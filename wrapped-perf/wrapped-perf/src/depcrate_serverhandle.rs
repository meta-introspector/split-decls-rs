// Generated macro for handle (function)
macro_rules! Depcrate_serverhandle {
() => {
// Module: crate::server
// Provides: {"handle"}
// Dependencies: {}
async fn handle (handshake : quinn :: Incoming , opt : Arc < Opt >) -> Result < () > { let connection = handshake . await . context ("handshake failed") ? ; debug ! ("{} connected" , connection . remote_address ()) ; tokio :: try_join ! (drive_uni (connection . clone ()) , drive_bi (connection . clone ()) , conn_stats (connection , opt)) ? ; Ok (()) }
};
}
