// Generated macro for TraceLevel (enum)
macro_rules! Depcrate_tracingTraceLevel {
() => {
// Module: crate::tracing
// Provides: {"TraceLevel"}
// Dependencies: {}
# [doc = " Available tracing levels.  When tracing is set to a particular level,"] # [doc = " callers will be provided tracing at the given level and all lower levels."] # [derive (Copy , Clone , Debug)] pub enum TraceLevel { # [doc = " No tracing will be performed."] None , # [doc = " Severe errors that may impact the program's execution"] Fatal , # [doc = " Errors that do not impact the program's execution"] Error , # [doc = " Warnings that suggest abnormal data"] Warn , # [doc = " Informational messages about program execution"] Info , # [doc = " Detailed data that allows for debugging"] Debug , # [doc = " Exceptionally detailed debugging data"] Trace , }
};
}
