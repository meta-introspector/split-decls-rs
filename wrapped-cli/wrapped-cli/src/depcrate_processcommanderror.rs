// Generated macro for CommandError (struct)
macro_rules! Depcrate_processCommandError {
() => {
// Module: crate::process
// Provides: {"CommandError"}
// Dependencies: {}
# [doc = " An error that can occur while running a command and reading its output."] # [doc = ""] # [doc = " This error can be seamlessly converted to an `io::Error` via a `From`"] # [doc = " implementation."] # [derive (Debug)] pub struct CommandError { kind : CommandErrorKind , }
};
}
