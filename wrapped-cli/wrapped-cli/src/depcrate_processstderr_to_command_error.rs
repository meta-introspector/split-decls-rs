// Generated macro for stderr_to_command_error (function)
macro_rules! Depcrate_processstderr_to_command_error {
() => {
// Module: crate::process
// Provides: {"stderr_to_command_error"}
// Dependencies: {}
fn stderr_to_command_error (stderr : & mut process :: ChildStderr) -> CommandError { let mut bytes = vec ! [] ; match stderr . read_to_end (& mut bytes) { Ok (_) => CommandError :: stderr (bytes) , Err (err) => CommandError :: io (err) , } }
};
}
