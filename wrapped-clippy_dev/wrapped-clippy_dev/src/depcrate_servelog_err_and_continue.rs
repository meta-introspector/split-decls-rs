// Generated macro for log_err_and_continue (function)
macro_rules! Depcrate_servelog_err_and_continue {
() => {
// Module: crate::serve
// Provides: {"log_err_and_continue"}
// Dependencies: {}
fn log_err_and_continue < T > (res : Result < T , impl Display > , path : & Path) -> Option < T > { match res { Ok (x) => Some (x) , Err (ref e) => { eprintln ! ("error reading `{}`: {e}" , path . display ()) ; None } , } }
};
}
