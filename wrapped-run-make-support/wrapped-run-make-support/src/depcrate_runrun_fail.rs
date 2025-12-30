// Generated macro for run_fail (function)
macro_rules! Depcrate_runrun_fail {
() => {
// Module: crate::run
// Provides: {"run_fail"}
// Dependencies: {}
# [doc = " Run a built binary and make sure it fails."] # [track_caller] pub fn run_fail (name : & str) -> CompletedProcess { let caller = panic :: Location :: caller () ; let mut cmd = run_common (name , None) ; let output = cmd . run_fail () ; if output . status () . success () { handle_failed_output (& cmd , output , caller . line ()) ; } output }
};
}
