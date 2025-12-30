// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> ExitCode { let config = parse_config (env :: args () . collect ()) ; let mut failed = Vec :: new () ; let mut cache = Cache :: new (& config) ; let Ok (directives) = get_directives (& config . template) else { eprintln ! ("Jsondocck failed for {}" , & config . template) ; return ExitCode :: FAILURE ; } ; for directive in directives { if let Err (message) = directive . check (& mut cache) { failed . push (CkError { directive , message }) ; } } if failed . is_empty () { ExitCode :: SUCCESS } else { for i in failed { eprintln ! ("{}:{}, directive failed" , config . template , i . directive . lineno) ; eprintln ! ("{}" , i . message) } ExitCode :: FAILURE } }
};
}
