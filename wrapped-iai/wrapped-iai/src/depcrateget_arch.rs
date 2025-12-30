// Generated macro for get_arch (function)
macro_rules! Depcrateget_arch {
() => {
// Module: crate
// Provides: {"get_arch"}
// Dependencies: {}
fn get_arch () -> String { let output = Command :: new ("uname") . arg ("-m") . stdout (Stdio :: piped ()) . output () . expect ("Failed to run `uname` to determine CPU architecture.") ; String :: from_utf8 (output . stdout) . expect ("`-uname -m` returned invalid unicode.") . trim () . to_owned () }
};
}
