// Generated macro for OutputMode (enum)
macro_rules! Depcrate_utils_execOutputMode {
() => {
// Module: crate::utils::exec
// Provides: {"OutputMode"}
// Dependencies: {}
# [doc = " How should the output of a specific stream of the command (stdout/stderr) be handled"] # [doc = " (whether it should be captured or printed)."] # [derive (Debug , Copy , Clone)] pub enum OutputMode { # [doc = " Prints the stream by inheriting it from the bootstrap process."] Print , # [doc = " Captures the stream into memory."] Capture , }
};
}
