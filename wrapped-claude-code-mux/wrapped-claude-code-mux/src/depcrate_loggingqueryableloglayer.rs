// Generated macro for QueryableLogLayer (struct)
macro_rules! Depcrate_loggingQueryableLogLayer {
() => {
// Module: crate::logging
// Provides: {"QueryableLogLayer"}
// Dependencies: {}
# [doc = " A tracing layer that stores logs in a ring buffer and on disk."] # [derive (Debug)] pub struct QueryableLogLayer { buffer : Arc < RwLock < VecDeque < LogEntry > > > , log_file : Arc < RwLock < File > > , }
};
}
