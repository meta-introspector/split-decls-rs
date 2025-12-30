// Generated macro for fetch (module)
macro_rules! Depcrate_driver_delayedfetch {
() => {
// Module: crate::driver::delayed
// Provides: {"fetch"}
// Dependencies: {}
# [doc = ""] pub mod fetch { use crate :: driver ; # [doc = " The error returned by [State::fetch_delayed()][super::State::fetch_delayed()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not get process named '{}' which should be running and tracked" , wanted . 0)] ProcessMissing { wanted : driver :: Key } , # [error ("Failed to run '{command}' command")] ProcessInvoke { command : String , source : driver :: process :: client :: invoke :: Error , } , # [error ("The invoked command '{command}' in process indicated an error: {status:?}")] ProcessStatus { status : driver :: process :: Status , command : String , } , } }
};
}
