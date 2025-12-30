// Generated macro for StderrReader (enum)
macro_rules! Depcrate_processStderrReader {
() => {
// Module: crate::process
// Provides: {"StderrReader"}
// Dependencies: {}
# [doc = " A reader that encapsulates the asynchronous or synchronous reading of"] # [doc = " stderr."] # [derive (Debug)] enum StderrReader { Async (Option < std :: thread :: JoinHandle < CommandError > >) , Sync (process :: ChildStderr) , }
};
}
