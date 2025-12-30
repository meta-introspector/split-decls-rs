// Generated macro for StderrForwarder (struct)
macro_rules! Depcrate_command_helpersStderrForwarder {
() => {
// Module: crate::command_helpers
// Provides: {"StderrForwarder"}
// Dependencies: {}
pub (crate) struct StderrForwarder { inner : Option < (ChildStderr , Vec < u8 >) > , # [cfg (feature = "parallel")] is_non_blocking : bool , # [cfg (feature = "parallel")] bytes_available_failed : bool , # [doc = " number of bytes buffered in inner"] bytes_buffered : usize , }
};
}
