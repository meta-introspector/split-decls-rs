// Generated macro for drive_bi (function)
macro_rules! Depcrate_serverdrive_bi {
() => {
// Module: crate::server
// Provides: {"drive_bi"}
// Dependencies: {}
async fn drive_bi (connection : quinn :: Connection) -> Result < () > { while let Ok ((send , recv)) = connection . accept_bi () . await { tokio :: spawn (async move { if let Err (e) = handle_bi (send , recv) . await { error ! ("request failed: {:#}" , e) ; } }) ; } Ok (()) }
};
}
