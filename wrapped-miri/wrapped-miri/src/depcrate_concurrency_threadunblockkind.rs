// Generated macro for UnblockKind (enum)
macro_rules! Depcrate_concurrency_threadUnblockKind {
() => {
// Module: crate::concurrency::thread
// Provides: {"UnblockKind"}
// Dependencies: {}
# [doc = " The argument type for the \"unblock\" callback, indicating why the thread got unblocked."] # [derive (Clone , Copy , Debug , PartialEq)] pub enum UnblockKind { # [doc = " Operation completed successfully, thread continues normal execution."] Ready , # [doc = " The operation did not complete within its specified duration."] TimedOut , }
};
}
