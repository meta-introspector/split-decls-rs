// Generated macro for ServerWorkerConfig (struct)
macro_rules! Depcrate_workerServerWorkerConfig {
() => {
// Module: crate::worker
// Provides: {"ServerWorkerConfig"}
// Dependencies: {}
# [doc = " Config for worker behavior passed down from server builder."] # [derive (Debug , Clone , Copy)] pub (crate) struct ServerWorkerConfig { shutdown_timeout : Duration , max_blocking_threads : usize , max_concurrent_connections : usize , }
};
}
