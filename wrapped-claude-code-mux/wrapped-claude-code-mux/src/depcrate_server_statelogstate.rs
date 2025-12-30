// Generated macro for LogState (struct)
macro_rules! Depcrate_server_stateLogState {
() => {
// Module: crate::server::state
// Provides: {"LogState"}
// Dependencies: {}
# [doc = " State for logging, including the in-memory buffer."] # [derive (Clone)] pub struct LogState { pub log_buffer : Arc < tokio :: sync :: RwLock < VecDeque < LogEntry > > > , pub log_file_path : String , }
};
}
