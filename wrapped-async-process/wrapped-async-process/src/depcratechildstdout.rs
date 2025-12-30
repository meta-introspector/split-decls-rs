// Generated macro for ChildStdout (struct)
macro_rules! DepcrateChildStdout {
() => {
// Module: crate
// Provides: {"ChildStdout"}
// Dependencies: {}
# [doc = " A handle to a child process's standard output (stdout)."] # [doc = ""] # [doc = " When a [`ChildStdout`] is dropped, the underlying handle gets closed."] # [derive (Debug)] pub struct ChildStdout (# [cfg (windows)] Unblock < std :: process :: ChildStdout > , # [cfg (unix)] Async < std :: process :: ChildStdout > ,) ;
};
}
