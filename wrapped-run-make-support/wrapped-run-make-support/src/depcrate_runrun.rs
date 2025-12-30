// Generated macro for run (function)
macro_rules! Depcrate_runrun {
() => {
// Module: crate::run
// Provides: {"run"}
// Dependencies: {}
# [doc = " Run a built binary and make sure it succeeds."] # [track_caller] pub fn run (name : & str) -> CompletedProcess { let caller = panic :: Location :: caller () ; let mut cmd = run_common (name , None) ; let output = cmd . run () ; if ! output . status () . success () { handle_failed_output (& cmd , output , caller . line ()) ; } output }
};
}
