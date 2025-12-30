// Generated macro for uname (function)
macro_rules! Depcrate_targetsuname {
() => {
// Module: crate::targets
// Provides: {"uname"}
// Dependencies: {}
# [doc = " Run `uname`. This assumes that `uname` is available on the platform!"] # [track_caller] # [must_use] pub fn uname () -> String { let caller = panic :: Location :: caller () ; let mut uname = Command :: new ("uname") ; let output = uname . run () ; if ! output . status () . success () { handle_failed_output (& uname , output , caller . line ()) ; } output . stdout_utf8 () }
};
}
