// Generated macro for ProcessSrvState (struct)
macro_rules! Depcrate_processProcessSrvState {
() => {
// Module: crate::process
// Provides: {"ProcessSrvState"}
// Dependencies: {}
# [doc = " Maintains the state of the proc-macro server process."] # [derive (Debug)] struct ProcessSrvState { process : Process , stdin : ChildStdin , stdout : BufReader < ChildStdout > , }
};
}
