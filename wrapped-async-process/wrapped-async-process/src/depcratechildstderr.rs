// Generated macro for ChildStderr (struct)
macro_rules! DepcrateChildStderr {
() => {
// Module: crate
// Provides: {"ChildStderr"}
// Dependencies: {}
# [doc = " A handle to a child process's standard error (stderr)."] # [doc = ""] # [doc = " When a [`ChildStderr`] is dropped, the underlying handle gets closed."] # [derive (Debug)] pub struct ChildStderr (# [cfg (windows)] Unblock < std :: process :: ChildStderr > , # [cfg (unix)] Async < std :: process :: ChildStderr > ,) ;
};
}
