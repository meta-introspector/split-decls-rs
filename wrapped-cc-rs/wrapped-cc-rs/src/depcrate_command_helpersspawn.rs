// Generated macro for spawn (function)
macro_rules! Depcrate_command_helpersspawn {
() => {
// Module: crate::command_helpers
// Provides: {"spawn"}
// Dependencies: {}
pub (crate) fn spawn (cmd : & mut Command , cargo_output : & CargoOutput) -> Result < Child , Error > { struct ResetStderr < 'cmd > (& 'cmd mut Command) ; impl Drop for ResetStderr < '_ > { fn drop (& mut self) { self . 0 . stderr (Stdio :: inherit ()) ; } } cargo_output . print_debug (& format_args ! ("running: {cmd:?}")) ; let cmd = ResetStderr (cmd) ; let child = cmd . 0 . stderr (cargo_output . stdio_for_warnings ()) . stdout (cargo_output . stdio_for_output ()) . spawn () ; match child { Ok (child) => Ok (child) , Err (ref e) if e . kind () == io :: ErrorKind :: NotFound => { let extra = if cfg ! (windows) { " (see https://docs.rs/cc/latest/cc/#compile-time-requirements for help)" } else { "" } ; Err (Error :: new (ErrorKind :: ToolNotFound , format ! ("failed to find tool {:?}: {e}{extra}" , cmd . 0 . get_program ()) ,)) } Err (e) => Err (Error :: new (ErrorKind :: ToolExecError , format ! ("command `{:?}` failed to start: {e}" , cmd . 0) ,)) , } }
};
}
