macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! invocation {
    () => {
        deps!();
        # [doc = ""] pub mod invocation { use std :: ffi :: OsString ; # [doc = " The error returned when producing ssh invocation arguments based on a selected invocation kind."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Username '{user}' could be mistaken for a command-line argument")] AmbiguousUserName { user : String } , # [error ("Host name '{host}' could be mistaken for a command-line argument")] AmbiguousHostName { host : String } , # [error ("The 'Simple' ssh variant doesn't support {function}")] Unsupported { # [doc = " The simple command that should have been invoked."] command : OsString , # [doc = " The function that was unsupported"] function : & 'static str , } , } }
    };
}

invocation!()