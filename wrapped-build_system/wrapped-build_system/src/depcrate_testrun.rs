// Generated macro for run (function)
macro_rules! Depcrate_testrun {
() => {
// Module: crate::test
// Provides: {"run"}
// Dependencies: {}
pub fn run () -> Result < () , String > { let mut args = match TestArg :: new () ? { Some (args) => args , None => return Ok (()) , } ; let mut env : HashMap < String , String > = std :: env :: vars () . collect () ; if ! args . use_system_gcc { args . config_info . setup_gcc_path () ? ; let gcc_path = args . config_info . gcc_path . clone () . expect ("The config module should have emitted an error if the GCC path wasn't provided" ,) ; env . insert ("LIBRARY_PATH" . to_string () , gcc_path . clone ()) ; env . insert ("LD_LIBRARY_PATH" . to_string () , gcc_path) ; } build_if_no_backend (& env , & args) ? ; if args . build_only { println ! ("Since it's build only, exiting...") ; return Ok (()) ; } args . config_info . setup (& mut env , args . use_system_gcc) ? ; if args . runners . is_empty () { run_all (& env , & args) ? ; } else { let runners = get_runners () ; for runner in args . runners . iter () { runners . get (runner . as_str ()) . unwrap () . 1 (& env , & args) ? ; } } Ok (()) }
};
}
