// Generated macro for extended_regex_tests (function)
macro_rules! Depcrate_testextended_regex_tests {
() => {
// Module: crate::test
// Provides: {"extended_regex_tests"}
// Dependencies: {}
fn extended_regex_tests (env : & Env , args : & TestArg) -> Result < () , String > { if ! args . is_using_gcc_master_branch () { println ! ("Not using GCC master branch. Skipping `extended_regex_tests`.") ; return Ok (()) ; } println ! ("[TEST] rust-lang/regex tests") ; let mut env = env . clone () ; let rustflags = format ! ("{} --cap-lints warn" , env . get ("RUSTFLAGS") . cloned () . unwrap_or_default ()) ; env . insert ("RUSTFLAGS" . to_string () , rustflags) ; let path = Path :: new (crate :: BUILD_DIR) . join ("regex") ; run_cargo_command (& [& "test" , & "--tests" , & "--" , & "--exclude-should-panic" , & "--test-threads" , & "1" , & "-Zunstable-options" , & "-q" ,] , Some (& path) , & env , args ,) ? ; Ok (()) }
};
}
