// Generated macro for build_if_no_backend (function)
macro_rules! Depcrate_testbuild_if_no_backend {
() => {
// Module: crate::test
// Provides: {"build_if_no_backend"}
// Dependencies: {}
fn build_if_no_backend (env : & Env , args : & TestArg) -> Result < () , String > { if args . config_info . backend . is_some () { return Ok (()) ; } let mut command : Vec < & dyn AsRef < OsStr > > = vec ! [& "cargo" , & "rustc"] ; let mut tmp_env ; let env = if args . config_info . channel == Channel :: Release { tmp_env = env . clone () ; tmp_env . insert ("CARGO_INCREMENTAL" . to_string () , "1" . to_string ()) ; command . push (& "--release") ; & tmp_env } else { env } ; for flag in args . flags . iter () { command . push (flag) ; } run_command_with_output_and_env (& command , None , Some (env)) }
};
}
