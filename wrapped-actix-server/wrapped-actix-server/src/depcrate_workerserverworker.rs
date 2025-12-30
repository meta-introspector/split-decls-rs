// Generated macro for ServerWorker (struct)
macro_rules! Depcrate_workerServerWorker {
() => {
// Module: crate::worker
// Provides: {"ServerWorker"}
// Dependencies: {}
# [doc = " Service worker."] # [doc = ""] # [doc = " Worker accepts Socket objects via unbounded channel and starts stream processing."] pub (crate) struct ServerWorker { conn_rx : UnboundedReceiver < Conn > , stop_rx : UnboundedReceiver < Stop > , counter : WorkerCounter , services : Box < [WorkerService] > , factories : Box < [Box < dyn InternalServiceFactory >] > , state : WorkerState , shutdown_timeout : Duration , }
};
}
