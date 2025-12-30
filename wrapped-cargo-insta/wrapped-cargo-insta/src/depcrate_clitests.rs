// Generated macro for tests (module)
macro_rules! Depcrate_clitests {
() => {
// Module: crate::cli
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn get_cargo_nextest_command_from_env_variables () { env :: set_var ("INSTA_CARGO_NEXTEST_BIN" , "/a/custom/path/to/cargo-nextest") ; let command = get_cargo_nextest_command () ; assert_eq ! (command . get_program () . to_string_lossy () , "/a/custom/path/to/cargo-nextest") ; assert_eq ! (command . get_args () . len () , 0) ; env :: remove_var ("INSTA_CARGO_NEXTEST_BIN") ; env :: set_var ("CARGO" , "/a/path/to/cargo") ; let command = get_cargo_nextest_command () ; assert_eq ! (command . get_program () . to_string_lossy () , "/a/path/to/cargo") ; let args : Vec < String > = command . get_args () . map (| arg | arg . to_string_lossy () . to_string ()) . collect () ; assert_eq ! (args , vec ! ["nextest"]) ; env :: remove_var ("CARGO") ; } }
};
}
