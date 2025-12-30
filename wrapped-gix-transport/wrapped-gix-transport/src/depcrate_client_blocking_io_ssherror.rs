// Generated macro for Error (enum)
macro_rules! Depcrate_client_blocking_io_sshError {
() => {
// Module: crate::client::blocking_io::ssh
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error used in [`connect()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("The scheme in \"{}\" is not usable for an ssh connection" , . 0 . to_bstring ())] UnsupportedScheme (gix_url :: Url) , # [error ("Host name '{host}' could be mistaken for a command-line argument")] AmbiguousHostName { host : String } , }
};
}
