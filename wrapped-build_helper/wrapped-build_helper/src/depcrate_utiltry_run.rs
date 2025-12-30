// Generated macro for try_run (function)
macro_rules! Depcrate_utiltry_run {
() => {
// Module: crate::util
// Provides: {"try_run"}
// Dependencies: {}
pub fn try_run (cmd : & mut Command , print_cmd_on_fail : bool) -> Result < () , () > { let status = match cmd . status () { Ok (status) => status , Err (e) => fail (& format ! ("failed to execute command: {cmd:?}\nerror: {e}")) , } ; if ! status . success () { if print_cmd_on_fail { println ! ("\n\ncommand did not execute successfully: {cmd:?}\n\
                 expected success, got: {status}\n\n") ; } Err (()) } else { Ok (()) } }
};
}
