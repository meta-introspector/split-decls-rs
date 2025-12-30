// Generated macro for WorkerHandleAccept (struct)
macro_rules! Depcrate_workerWorkerHandleAccept {
() => {
// Module: crate::worker
// Provides: {"WorkerHandleAccept"}
// Dependencies: {}
# [doc = " Handle to worker that can send connection message to worker and share the availability of worker"] # [doc = " to other threads."] # [doc = ""] # [doc = " Held by [Accept](crate::accept::Accept)."] pub (crate) struct WorkerHandleAccept { idx : usize , conn_tx : UnboundedSender < Conn > , counter : Counter , }
};
}
