// Generated macro for ProgressMonitor (struct)
macro_rules! Depcrate_progressProgressMonitor {
() => {
// Module: crate::progress
// Provides: {"ProgressMonitor"}
// Dependencies: {}
# [doc = " A generic wrapper around a reader that keeps track of how many bytes have been read"] # [doc = " from the total."] # [doc = ""] # [doc = " This wrapper has a lock on standard out for the lifetime of the monitor"] pub struct ProgressMonitor < R : Read > { # [doc = " The total amount that the reader will read"] pub total : usize , # [doc = " Amount read so far"] pub read : usize , # [doc = " The internal reader"] reader : R , progress_bar : ProgressBar , }
};
}
