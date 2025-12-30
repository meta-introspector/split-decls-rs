// Generated macro for run_with_output (function)
macro_rules! Depcrate_utilsrun_with_output {
() => {
// Module: crate::utils
// Provides: {"run_with_output"}
// Dependencies: {}
# [track_caller] # [must_use] pub fn run_with_output (path : & (impl AsRef < Path > + ? Sized) , cmd : & mut Command) -> Vec < u8 > { fn f (path : & Path , cmd : & mut Command) -> Vec < u8 > { let output = expect_action (cmd . stdin (Stdio :: null ()) . stdout (Stdio :: piped ()) . stderr (Stdio :: inherit ()) . output () , ErrAction :: Run , path ,) ; expect_action (output . status . exit_ok () , ErrAction :: Run , path) ; output . stdout } f (path . as_ref () , cmd) }
};
}
