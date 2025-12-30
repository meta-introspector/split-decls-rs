// Generated macro for uname_cli (function)
macro_rules! Depcrate_unameuname_cli {
() => {
// Module: crate::uname
// Provides: {"uname_cli"}
// Dependencies: {}
fn uname_cli (arg : & str) -> Option < String > { Command :: new ("uname") . arg (arg) . output () . map_err (| e | { error ! ("Failed to invoke 'uname {}': {:?}" , arg , e) ; }) . ok () . and_then (| out | { if out . status . success () { Some (String :: from_utf8_lossy (& out . stdout) . trim_end () . to_owned ()) } else { error ! ("'uname {}' failed with status: {:?}" , arg , out . status) ; None } }) }
};
}
