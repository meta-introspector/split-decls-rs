// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_errorErrorKind {
() => {
// Module: crate::error
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " For programmatically processing failures."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum ErrorKind { # [doc = " Spawning the cargo subommand failed."] InvalidCommand , # [doc = " The cargo subcommand returned an error."] CommandFailed , # [doc = " Parsing the cargo subcommand's output failed."] InvalidOutput , }
};
}
