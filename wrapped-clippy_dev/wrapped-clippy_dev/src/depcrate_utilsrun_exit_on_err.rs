// Generated macro for run_exit_on_err (function)
macro_rules! Depcrate_utilsrun_exit_on_err {
() => {
// Module: crate::utils
// Provides: {"run_exit_on_err"}
// Dependencies: {}
# [track_caller] pub fn run_exit_on_err (path : & (impl AsRef < Path > + ? Sized) , cmd : & mut Command) { match expect_action (cmd . status () , ErrAction :: Run , path . as_ref ()) . code () { Some (0) => { } , Some (n) => process :: exit (n) , None => { eprintln ! ("{} killed by signal" , path . as_ref () . display ()) ; process :: exit (1) ; } , } }
};
}
