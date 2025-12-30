// Generated macro for ChildStdin (struct)
macro_rules! DepcrateChildStdin {
() => {
// Module: crate
// Provides: {"ChildStdin"}
// Dependencies: {}
# [doc = " A handle to a child process's standard input (stdin)."] # [doc = ""] # [doc = " When a [`ChildStdin`] is dropped, the underlying handle gets closed. If the child process was"] # [doc = " previously blocked on input, it becomes unblocked after dropping."] # [derive (Debug)] pub struct ChildStdin (# [cfg (windows)] Unblock < std :: process :: ChildStdin > , # [cfg (unix)] Async < std :: process :: ChildStdin > ,) ;
};
}
