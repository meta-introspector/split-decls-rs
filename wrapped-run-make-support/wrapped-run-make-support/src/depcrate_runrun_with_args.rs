// Generated macro for run_with_args (function)
macro_rules! Depcrate_runrun_with_args {
() => {
// Module: crate::run
// Provides: {"run_with_args"}
// Dependencies: {}
# [doc = " Run a built binary with one or more argument(s) and make sure it succeeds."] # [track_caller] pub fn run_with_args (name : & str , args : & [& str]) -> CompletedProcess { let caller = panic :: Location :: caller () ; let mut cmd = run_common (name , Some (args)) ; let output = cmd . run () ; if ! output . status () . success () { handle_failed_output (& cmd , output , caller . line ()) ; } output }
};
}
