// Generated macro for try_wait_on_child (function)
macro_rules! Depcrate_parallel_command_runnertry_wait_on_child {
() => {
// Module: crate::parallel::command_runner
// Provides: {"try_wait_on_child"}
// Dependencies: {}
fn try_wait_on_child (cmd : & Command , child : & mut Child , mut stdout : impl io :: Write , stderr_forwarder : & mut StderrForwarder ,) -> Result < Option < () > , Error > { stderr_forwarder . forward_available () ; match child . try_wait () { Ok (Some (status)) => { stderr_forwarder . forward_all () ; let _ = writeln ! (stdout , "{}" , status) ; if status . success () { Ok (Some (())) } else { Err (Error :: new (ErrorKind :: ToolExecError , format ! ("command did not execute successfully (status code {status}): {cmd:?}") ,)) } } Ok (None) => Ok (None) , Err (e) => { stderr_forwarder . forward_all () ; Err (Error :: new (ErrorKind :: ToolExecError , format ! ("failed to wait on spawned child process `{cmd:?}`: {e}") ,)) } } }
};
}
