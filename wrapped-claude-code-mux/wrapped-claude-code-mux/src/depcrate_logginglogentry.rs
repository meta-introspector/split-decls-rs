// Generated macro for LogEntry (struct)
macro_rules! Depcrate_loggingLogEntry {
() => {
// Module: crate::logging
// Provides: {"LogEntry"}
// Dependencies: {}
# [doc = " A structured log entry."] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct LogEntry { pub timestamp : DateTime < Utc > , pub level : String , pub target : String , pub message : String , }
};
}
