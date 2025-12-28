macro_rules! deps {
    () => {
        Status!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " The error returned by [State::apply()][super::State::apply()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Init (# [from] driver :: init :: Error) , # [error ("Could not write entire object to driver")] WriteSource (# [from] std :: io :: Error) , # [error ("Filter process delayed an entry even though that was not requested")] DelayNotAllowed , # [error ("Failed to invoke '{command}' command")] ProcessInvoke { source : process :: client :: invoke :: Error , command : String , } , # [error ("The invoked command '{command}' in process indicated an error: {status:?}")] ProcessStatus { status : driver :: process :: Status , command : String , } , }
    };
}

Error!();