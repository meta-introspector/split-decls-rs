// Generated macro for drive_uni (function)
macro_rules! Depcrate_serverdrive_uni {
() => {
// Module: crate::server
// Provides: {"drive_uni"}
// Dependencies: {}
async fn drive_uni (connection : quinn :: Connection) -> Result < () > { while let Ok (stream) = connection . accept_uni () . await { let connection = connection . clone () ; tokio :: spawn (async move { if let Err (e) = handle_uni (connection , stream) . await { error ! ("request failed: {:#}" , e) ; } }) ; } Ok (()) }
};
}
