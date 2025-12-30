// Generated macro for WorkerHandleServer (struct)
macro_rules! Depcrate_workerWorkerHandleServer {
() => {
// Module: crate::worker
// Provides: {"WorkerHandleServer"}
// Dependencies: {}
# [doc = " Handle to worker than can send stop message to worker."] # [doc = ""] # [doc = " Held by [ServerBuilder](crate::builder::ServerBuilder)."] # [derive (Debug)] pub (crate) struct WorkerHandleServer { pub (crate) idx : usize , stop_tx : UnboundedSender < Stop > , }
};
}
