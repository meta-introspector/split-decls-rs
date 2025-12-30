// Generated macro for IOResourceError (enum)
macro_rules! Depcrate_connectionIOResourceError {
() => {
// Module: crate::connection
// Provides: {"IOResourceError"}
// Dependencies: {}
# [doc = " An error that can occur in the dbus-tokio reactor"] # [derive (Debug)] # [non_exhaustive] pub enum IOResourceError { # [doc = " An error that occurred while interacting with dbus"] Dbus (dbus :: Error) , # [doc = " An error that likely occurred on tokio's side"] Io (io :: Error) , }
};
}
