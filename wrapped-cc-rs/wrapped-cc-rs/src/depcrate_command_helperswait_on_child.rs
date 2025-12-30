// Generated macro for wait_on_child (function)
macro_rules! Depcrate_command_helperswait_on_child {
() => {
// Module: crate::command_helpers
// Provides: {"wait_on_child"}
// Dependencies: {}
fn wait_on_child (cmd : & Command , child : & mut Child , cargo_output : & CargoOutput ,) -> Result < () , Error > { StderrForwarder :: new (child) . forward_all () ; let status = match child . wait () { Ok (s) => s , Err (e) => { return Err (Error :: new (ErrorKind :: ToolExecError , format ! ("failed to wait on spawned child process `{cmd:?}`: {e}") ,)) ; } } ; cargo_output . print_debug (& status) ; if status . success () { Ok (()) } else { Err (Error :: new (ErrorKind :: ToolExecError , format ! ("command did not execute successfully (status code {status}): {cmd:?}") ,)) } }
};
}
