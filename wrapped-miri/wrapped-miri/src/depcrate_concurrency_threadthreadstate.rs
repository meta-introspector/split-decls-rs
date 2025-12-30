// Generated macro for ThreadState (enum)
macro_rules! Depcrate_concurrency_threadThreadState {
() => {
// Module: crate::concurrency::thread
// Provides: {"ThreadState"}
// Dependencies: {}
# [doc = " The state of a thread."] enum ThreadState < 'tcx > { # [doc = " The thread is enabled and can be executed."] Enabled , # [doc = " The thread is blocked on something."] Blocked { reason : BlockReason , timeout : Option < Timeout > , callback : DynUnblockCallback < 'tcx > } , # [doc = " The thread has terminated its execution. We do not delete terminated"] # [doc = " threads (FIXME: why?)."] Terminated , }
};
}
