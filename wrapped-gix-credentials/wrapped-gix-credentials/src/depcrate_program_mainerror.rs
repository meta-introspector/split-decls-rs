// Generated macro for Error (enum)
macro_rules! Depcrate_program_mainError {
() => {
// Module: crate::program::main
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error of [`main()`][crate::program::main()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Action named {name:?} is invalid, need 'get', 'store', 'erase' or 'fill', 'approve', 'reject'")] ActionInvalid { name : OsString } , # [error ("The first argument must be the action to perform")] ActionMissing , # [error (transparent)] Helper { source : Box < dyn std :: error :: Error + Send + Sync + 'static > , } , # [error (transparent)] Io (# [from] std :: io :: Error) , # [error (transparent)] Context (# [from] crate :: protocol :: context :: decode :: Error) , # [error ("Credentials for {url:?} could not be obtained")] CredentialsMissing { url : BString } , # [error ("Either 'url' field or both 'protocol' and 'host' fields must be provided")] UrlMissing , }
};
}
