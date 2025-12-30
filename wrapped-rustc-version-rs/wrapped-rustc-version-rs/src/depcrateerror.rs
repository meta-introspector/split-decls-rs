// Generated macro for Error (enum)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error type for this crate."] # [derive (Debug)] pub enum Error { # [doc = " An error occurred while trying to find the `rustc` to run."] CouldNotExecuteCommand (io :: Error) , # [doc = " Error output from the command that was run."] CommandError { # [doc = " stdout output from the command"] stdout : String , # [doc = " stderr output from the command"] stderr : String , } , # [doc = " The output of `rustc -vV` was not valid utf-8."] Utf8Error (str :: Utf8Error) , # [doc = " The output of `rustc -vV` was not in the expected format."] UnexpectedVersionFormat , # [doc = " An error occurred in parsing the semver."] SemVerError (semver :: Error) , # [doc = " The pre-release tag is unknown."] UnknownPreReleaseTag (String) , # [doc = " An error occurred in parsing a `LlvmVersion`."] LlvmVersionError (LlvmVersionParseError) , }
};
}
