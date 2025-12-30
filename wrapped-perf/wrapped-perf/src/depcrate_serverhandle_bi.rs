// Generated macro for handle_bi (function)
macro_rules! Depcrate_serverhandle_bi {
() => {
// Module: crate::server
// Provides: {"handle_bi"}
// Dependencies: {}
async fn handle_bi (send : quinn :: SendStream , recv : quinn :: RecvStream) -> Result < () > { let bytes = read_req (recv) . await ? ; respond (bytes , send) . await ? ; Ok (()) }
};
}
