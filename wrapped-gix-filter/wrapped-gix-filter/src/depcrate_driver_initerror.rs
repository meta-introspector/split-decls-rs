// Generated macro for Error (enum)
macro_rules! Depcrate_driver_initError {
() => {
// Module: crate::driver::init
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [State::maybe_launch_process()][super::State::maybe_launch_process()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Failed to spawn driver: {command:?}")] SpawnCommand { source : std :: io :: Error , command : std :: process :: Command , } , # [error ("Process handshake with command {command:?} failed")] ProcessHandshake { source : process :: client :: handshake :: Error , command : std :: process :: Command , } , }
};
}
