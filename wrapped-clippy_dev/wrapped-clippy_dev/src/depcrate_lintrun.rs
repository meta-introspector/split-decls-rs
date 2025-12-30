// Generated macro for run (function)
macro_rules! Depcrate_lintrun {
() => {
// Module: crate::lint
// Provides: {"run"}
// Dependencies: {}
pub fn run < 'a > (path : & str , edition : & str , args : impl Iterator < Item = & 'a String >) { let is_file = expect_action (fs :: metadata (path) , ErrAction :: Read , path) . is_file () ; if is_file { run_exit_on_err ("cargo run" , cargo_cmd () . args (["run" , "--bin" , "clippy-driver" , "--"]) . args (["-L" , "./target/debug"]) . args (["-Z" , "no-codegen"]) . args (["--edition" , edition]) . arg (path) . args (args) . env ("RUSTC_ICE" , "0") ,) ; } else { run_exit_on_err ("cargo build" , cargo_cmd () . arg ("build")) ; let mut exe = env :: current_exe () . expect ("failed to get current executable name") ; exe . set_file_name (CARGO_CLIPPY_EXE) ; run_exit_on_err ("cargo clippy" , Command :: new (exe) . arg ("clippy") . args (args) . env ("RUSTC_ICE" , "0") . current_dir (path) ,) ; } }
};
}
