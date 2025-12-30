// Generated macro for handle_uni (function)
macro_rules! Depcrate_serverhandle_uni {
() => {
// Module: crate::server
// Provides: {"handle_uni"}
// Dependencies: {}
async fn handle_uni (connection : quinn :: Connection , stream : quinn :: RecvStream) -> Result < () > { let bytes = read_req (stream) . await ? ; let response = connection . open_uni () . await ? ; respond (bytes , response) . await ? ; Ok (()) }
};
}
