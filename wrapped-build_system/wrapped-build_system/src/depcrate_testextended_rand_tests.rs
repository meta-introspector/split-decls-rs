// Generated macro for extended_rand_tests (function)
macro_rules! Depcrate_testextended_rand_tests {
() => {
// Module: crate::test
// Provides: {"extended_rand_tests"}
// Dependencies: {}
fn extended_rand_tests (env : & Env , args : & TestArg) -> Result < () , String > { if ! args . is_using_gcc_master_branch () { println ! ("Not using GCC master branch. Skipping `extended_rand_tests`.") ; return Ok (()) ; } let mut env = env . clone () ; let rustflags = format ! ("{} --cap-lints warn" , env . get ("RUSTFLAGS") . cloned () . unwrap_or_default ()) ; env . insert ("RUSTFLAGS" . to_string () , rustflags) ; let path = Path :: new (crate :: BUILD_DIR) . join ("rand") ; run_cargo_command (& [& "clean"] , Some (& path) , & env , args) ? ; println ! ("[TEST] rust-random/rand") ; run_cargo_command (& [& "test" , & "--workspace"] , Some (& path) , & env , args) ? ; Ok (()) }
};
}
