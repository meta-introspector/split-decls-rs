// Generated macro for debug_with_rustfmt (function)
macro_rules! Depcrate_utilsdebug_with_rustfmt {
() => {
// Module: crate::utils
// Provides: {"debug_with_rustfmt"}
// Dependencies: {}
# [cfg (feature = "debug-with-rustfmt")] # [doc = " Pretty-print the output of proc macro using rustfmt."] pub fn debug_with_rustfmt (input : & TokenStream) { use std :: env ; use std :: ffi :: OsStr ; use std :: io :: Write ; use std :: process :: { Command , Stdio } ; let rustfmt_var = env :: var_os ("RUSTFMT") ; let rustfmt = match & rustfmt_var { Some (rustfmt) => rustfmt , None => OsStr :: new ("rustfmt") , } ; let mut child = Command :: new (rustfmt) . stdin (Stdio :: piped ()) . stdout (Stdio :: piped ()) . spawn () . expect ("Failed to spawn rustfmt in stdio mode") ; { let stdin = child . stdin . as_mut () . expect ("Failed to get stdin") ; stdin . write_all (format ! ("{}" , input) . as_bytes ()) . expect ("Failed to write to stdin") ; } let rustfmt_output = child . wait_with_output () . expect ("rustfmt has failed") ; eprintln ! ("{}" , String :: from_utf8 (rustfmt_output . stdout) . expect ("rustfmt returned non-UTF8 string")) ; }
};
}
