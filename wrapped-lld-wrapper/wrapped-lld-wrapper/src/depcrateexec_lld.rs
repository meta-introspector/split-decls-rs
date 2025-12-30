// Generated macro for exec_lld (function)
macro_rules! Depcrateexec_lld {
() => {
// Module: crate
// Provides: {"exec_lld"}
// Dependencies: {}
# [cfg (not (unix))] fn exec_lld (mut command : process :: Command) { let exit_status = command . status () . unwrap_or_exit_with ("error running rust-lld child process") ; let code = exit_status . code () . ok_or (exit_status) . unwrap_or_exit_with ("rust-lld child process exited with error") ; process :: exit (code) ; }
};
}
