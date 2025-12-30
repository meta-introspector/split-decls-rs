// Generated macro for cargo_tests (function)
macro_rules! Depcrate_testcargo_tests {
() => {
// Module: crate::test
// Provides: {"cargo_tests"}
// Dependencies: {}
fn cargo_tests (test_env : & Env , test_args : & TestArg) -> Result < () , String > { mini_tests (test_env , test_args) ? ; let mut env = HashMap :: new () ; env . insert ("LD_LIBRARY_PATH" . into () , test_env . get ("LD_LIBRARY_PATH") . expect ("LD_LIBRARY_PATH missing!") . to_string () ,) ; env . insert ("LIBRARY_PATH" . into () , test_env . get ("LIBRARY_PATH") . expect ("LIBRARY_PATH missing!") . to_string () ,) ; env . insert ("CG_RUSTFLAGS" . into () , test_env . get ("CG_RUSTFLAGS") . map (| s | s . as_str ()) . unwrap_or ("") . to_string () ,) ; let mut args : Vec < & dyn AsRef < OsStr > > = vec ! [& "cargo" , & "test"] ; args . extend (test_args . test_args . iter () . map (| s | s as & dyn AsRef < OsStr >)) ; run_command_with_output_and_env (& args , None , Some (& env)) ? ; Ok (()) }
};
}
