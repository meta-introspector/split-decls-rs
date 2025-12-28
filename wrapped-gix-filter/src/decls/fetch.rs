macro_rules! deps {
    () => {
        Status!();
        Key!();
        Error!();
    };
}

macro_rules! fetch {
    () => {
        deps!();
        # [doc = ""] pub mod fetch { use crate :: driver ; # [doc = " The error returned by [State::fetch_delayed()][super::State::fetch_delayed()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not get process named '{}' which should be running and tracked" , wanted . 0)] ProcessMissing { wanted : driver :: Key } , # [error ("Failed to run '{command}' command")] ProcessInvoke { command : String , source : driver :: process :: client :: invoke :: Error , } , # [error ("The invoked command '{command}' in process indicated an error: {status:?}")] ProcessStatus { status : driver :: process :: Status , command : String , } , } }
    };
}

fetch!()