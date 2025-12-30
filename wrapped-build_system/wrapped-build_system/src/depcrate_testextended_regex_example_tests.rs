// Generated macro for extended_regex_example_tests (function)
macro_rules! Depcrate_testextended_regex_example_tests {
() => {
// Module: crate::test
// Provides: {"extended_regex_example_tests"}
// Dependencies: {}
fn extended_regex_example_tests (env : & Env , args : & TestArg) -> Result < () , String > { if ! args . is_using_gcc_master_branch () { println ! ("Not using GCC master branch. Skipping `extended_regex_example_tests`.") ; return Ok (()) ; } let path = Path :: new (crate :: BUILD_DIR) . join ("regex") ; run_cargo_command (& [& "clean"] , Some (& path) , env , args) ? ; println ! ("[TEST] rust-lang/regex example shootout-regex-dna") ; let mut env = env . clone () ; let rustflags = format ! ("{} --cap-lints warn" , env . get ("RUSTFLAGS") . cloned () . unwrap_or_default ()) ; env . insert ("RUSTFLAGS" . to_string () , rustflags) ; run_cargo_command (& [& "build" , & "--example" , & "shootout-regex-dna"] , Some (& path) , & env , args) ? ; run_cargo_command_with_callback (& [& "run" , & "--example" , & "shootout-regex-dna"] , Some (& path) , & env , args , | cargo_command , cwd , env | { let mut command : Vec < & dyn AsRef < OsStr > > = vec ! [& "bash" , & "-c"] ; let cargo_args = cargo_command . iter () . map (| s | s . as_ref () . to_str () . unwrap ()) . collect :: < Vec < _ > > () ; let bash_command = format ! ("cat examples/regexdna-input.txt | {} | grep -v 'Spawned thread' > res.txt" , cargo_args . join (" ") ,) ; command . push (& bash_command) ; run_command_with_output_and_env (& command , cwd , Some (env)) ? ; run_command_with_output_and_env (& [& "diff" , & "-u" , & "res.txt" , & "examples/regexdna-output.txt"] , cwd , Some (env) ,) ? ; Ok (()) } ,) ? ; Ok (()) }
};
}
