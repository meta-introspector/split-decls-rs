// Generated macro for exec_with_pipe (function)
macro_rules! Depcrate_utilexec_with_pipe {
() => {
// Module: crate::util
// Provides: {"exec_with_pipe"}
// Dependencies: {}
# [doc = " Execute the `Command`, then exit this process with the exit code of the new process."] # [doc = " `input` is also piped to the new process's stdin."] pub fn exec_with_pipe (mut cmd : Command , input : & [u8]) -> ! { cmd . stdin (std :: process :: Stdio :: piped ()) ; let mut child = cmd . spawn () . expect ("failed to spawn process") ; let child_stdin = child . stdin . take () . unwrap () ; let exit_status = std :: thread :: scope (| s | { s . spawn (| | { let mut child_stdin = child_stdin ; let _ = child_stdin . write_all (input) ; }) ; child . wait () . expect ("failed to run command") }) ; std :: process :: exit (exit_status . code () . unwrap_or (- 1)) }
};
}
