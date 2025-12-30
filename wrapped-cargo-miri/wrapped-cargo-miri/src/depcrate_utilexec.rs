// Generated macro for exec (function)
macro_rules! Depcrate_utilexec {
() => {
// Module: crate::util
// Provides: {"exec"}
// Dependencies: {}
# [doc = " Execute the `Command`, where possible by replacing the current process with a new process"] # [doc = " described by the `Command`. Then exit this process with the exit code of the new process."] pub fn exec (mut cmd : Command) -> ! { # [cfg (not (unix))] { let exit_status = cmd . status () . unwrap_or_else (| err | panic ! ("failed to run `{cmd:?}`:\n{err}")) ; std :: process :: exit (exit_status . code () . unwrap_or (- 1)) } # [cfg (unix)] { use std :: os :: unix :: process :: CommandExt ; let err = cmd . exec () ; panic ! ("failed to run `{cmd:?}`:\n{err}") } }
};
}
