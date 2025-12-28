macro_rules! deps {
    () => {
        Error!();
        Status!();
        Key!();
    };
}

macro_rules! list {
    () => {
        deps!();
        # [doc = ""] pub mod list { use crate :: driver ; # [doc = " The error returned by [State::list_delayed_paths()][super::State::list_delayed_paths()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not get process named '{}' which should be running and tracked" , wanted . 0)] ProcessMissing { wanted : driver :: Key } , # [error ("Failed to run 'list_available_blobs' command")] ProcessInvoke (# [from] driver :: process :: client :: invoke :: without_content :: Error) , # [error ("The invoked command 'list_available_blobs' in process indicated an error: {status:?}")] ProcessStatus { status : driver :: process :: Status } , } }
    };
}

list!()