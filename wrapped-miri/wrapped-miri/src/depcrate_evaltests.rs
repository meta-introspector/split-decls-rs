// Generated macro for tests (module)
macro_rules! Depcrate_evaltests {
() => {
// Module: crate::eval
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] # [should_panic (expected = "argv[0] cannot contain a doublequote (\") character")] fn windows_argv0_panic_on_quote () { args_to_utf16_command_string (["\""] . iter ()) ; } # [test] fn windows_argv0_no_escape () { let cmd = String :: from_utf16_lossy (& args_to_utf16_command_string ([r"C:\Program Files\" , "arg1" , "arg 2" , "arg \" 3"] . iter () ,)) ; assert_eq ! (cmd . trim_end_matches ('\0') , r#""C:\Program Files\" arg1 "arg 2" "arg \" 3""#) ; } }
};
}
