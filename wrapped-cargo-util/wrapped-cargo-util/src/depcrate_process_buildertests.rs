// Generated macro for tests (module)
macro_rules! Depcrate_process_buildertests {
() => {
// Module: crate::process_builder
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: ProcessBuilder ; use std :: fs ; # [test] fn argfile_build_succeeds () { let mut cmd = ProcessBuilder :: new ("echo") ; cmd . args (["foo" , "bar"] . as_slice ()) ; let (cmd , argfile) = cmd . build_command_with_argfile () . unwrap () ; assert_eq ! (cmd . get_program () , "echo") ; let cmd_args : Vec < _ > = cmd . get_args () . map (| s | s . to_str () . unwrap ()) . collect () ; assert_eq ! (cmd_args . len () , 1) ; assert ! (cmd_args [0] . starts_with ("@")) ; assert ! (cmd_args [0] . contains ("cargo-argfile.")) ; let buf = fs :: read_to_string (argfile . path ()) . unwrap () ; assert_eq ! (buf , "foo\nbar\n") ; } # [test] fn argfile_build_fails_if_arg_contains_newline () { let mut cmd = ProcessBuilder :: new ("echo") ; cmd . arg ("foo\n") ; let err = cmd . build_command_with_argfile () . unwrap_err () ; assert_eq ! (err . to_string () , "argument for argfile contains newlines: `foo\n`") ; } # [test] fn argfile_build_fails_if_arg_contains_invalid_utf8 () { let mut cmd = ProcessBuilder :: new ("echo") ; # [cfg (windows)] let invalid_arg = { use std :: os :: windows :: prelude :: * ; std :: ffi :: OsString :: from_wide (& [0x0066 , 0x006f , 0xD800 , 0x006f]) } ; # [cfg (unix)] let invalid_arg = { use std :: os :: unix :: ffi :: OsStrExt ; std :: ffi :: OsStr :: from_bytes (& [0x66 , 0x6f , 0x80 , 0x6f]) . to_os_string () } ; cmd . arg (invalid_arg) ; let err = cmd . build_command_with_argfile () . unwrap_err () ; assert_eq ! (err . to_string () , "argument for argfile contains invalid UTF-8 characters: `fo�o`") ; } }
};
}
